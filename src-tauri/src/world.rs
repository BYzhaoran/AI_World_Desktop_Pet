use chrono::{DateTime, Local};
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

const EVENT_TYPES: &[&str] = &[
    "normal_event", "social_event", "activity_event", "weather_event",
    "discovery_event", "item_event", "skill_event", "relationship_event",
    "important_event", "milestone_event", "level_up",
];

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelationshipEffect { pub target: String, pub delta: i32 }

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct PersonalitySignal { pub trait_name: String, pub delta: i32, pub reason: String }

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct EventEffects {
    pub energy: i32,
    #[serde(alias = "happiness")]
    pub mood: i32,
    pub xp: i32,
    pub relationship: Option<RelationshipEffect>,
    pub personality_signal: Option<PersonalitySignal>,
    pub item: Option<ItemEffect>,
    pub skill: Option<SkillEffect>,
    pub goal: Option<GoalEffect>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemEffect { pub id: String, pub name: String, pub description: String, pub quantity: i32 }

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillEffect { pub id: String, pub name: String, pub experience: i32 }

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoalEffect { pub id: String, pub progress: i32 }

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct EventProposal {
    #[serde(alias = "event_type", alias = "type")]
    pub event_type: String,
    pub summary: String,
    pub importance: f32,
    pub location: String,
    pub effects: EventEffects,
    pub participants: Vec<String>,
    pub causes: Vec<String>,
    pub memory: bool,
}

impl Default for EventProposal {
    fn default() -> Self { Self::no_event() }
}

impl EventProposal {
    pub fn no_event() -> Self {
        Self { event_type: "no_event".into(), summary: String::new(), importance: 0.0,
            location: String::new(), effects: EventEffects::default(), participants: vec![], causes: vec![], memory: false }
    }
    pub fn xp_delta(&self) -> i32 { self.effects.xp }
    pub fn relationship(&self) -> Option<&RelationshipEffect> { self.effects.relationship.as_ref() }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventRecord {
    pub id: String, pub timestamp: String, #[serde(rename = "type")] pub event_type: String,
    pub summary: String, pub importance: f32, pub location: String,
    pub participants: Vec<String>, pub causes: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorldSnapshot {
    pub name: String, pub level: i32, pub xp: i32, pub next_xp: i32,
    pub mood: i32, pub energy: i32, pub location: String, pub weather: String,
    pub status: String, pub animation: String,
    pub traits: Vec<Trait>, pub skills: Vec<Skill>, pub inventory: Vec<InventoryItem>,
    pub goals: Vec<Goal>, pub npcs: Vec<Npc>, pub events: Vec<EventRecord>,
    pub world_time: String, pub last_update: String, pub important_today: i32,
    pub next_normal_check: Option<i64>, pub memory_context: String,
    pub memories: Vec<String>, pub personality_evidence: Vec<PersonalityEvidence>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Trait { pub name: String, pub score: i32, pub color: String }
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Skill { pub name: String, pub level: i32, pub xp: i32 }
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InventoryItem { pub name: String, pub detail: String, pub icon: String }
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Goal { pub name: String, pub progress: i32, pub target: i32 }
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Npc { pub id: String, pub name: String, pub role: String, pub relationship: i32, pub stage: String, pub avatar: String }
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonalityEvidence { pub trait_name: String, pub delta: i32, pub event_id: String, pub reason: String }

pub struct Engine { db: Connection, root: PathBuf }

impl Engine {
    pub fn open() -> rusqlite::Result<Self> {
        let root = dirs_path();
        eprintln!("[world] data_root={}", root.display());
        fs::create_dir_all(&root).ok();
        let db = Connection::open(root.join("world.sqlite3"))?;
        let mut engine = Self { db, root };
        engine.migrate()?;
        engine.seed_defaults()?;
        engine.sync_from_markdown()?;
        Ok(engine)
    }

    fn migrate(&self) -> rusqlite::Result<()> {
        self.db.execute_batch("CREATE TABLE IF NOT EXISTS world_state (key TEXT PRIMARY KEY,value TEXT NOT NULL); CREATE TABLE IF NOT EXISTS characters (id TEXT PRIMARY KEY,name TEXT NOT NULL,level INTEGER NOT NULL,xp INTEGER NOT NULL); CREATE TABLE IF NOT EXISTS personality_traits (name TEXT PRIMARY KEY,score INTEGER NOT NULL,color TEXT NOT NULL); CREATE TABLE IF NOT EXISTS events (id TEXT PRIMARY KEY,timestamp TEXT NOT NULL,type TEXT NOT NULL,summary TEXT NOT NULL,importance REAL NOT NULL,location TEXT NOT NULL,causes TEXT NOT NULL,participants TEXT NOT NULL DEFAULT '[]'); CREATE TABLE IF NOT EXISTS personality_evidence (id INTEGER PRIMARY KEY,trait TEXT NOT NULL,delta INTEGER NOT NULL,event_id TEXT NOT NULL,reason TEXT NOT NULL); CREATE TABLE IF NOT EXISTS relationships (npc_id TEXT PRIMARY KEY,score INTEGER NOT NULL,stage TEXT NOT NULL); CREATE TABLE IF NOT EXISTS shared_experiences (id INTEGER PRIMARY KEY,event_ids TEXT NOT NULL,summary TEXT NOT NULL); CREATE TABLE IF NOT EXISTS memories (id TEXT PRIMARY KEY,event_id TEXT NOT NULL,summary TEXT NOT NULL,created_at TEXT NOT NULL); CREATE TABLE IF NOT EXISTS npcs (id TEXT PRIMARY KEY,name TEXT NOT NULL,role TEXT NOT NULL,avatar TEXT NOT NULL); CREATE TABLE IF NOT EXISTS important_people (id TEXT PRIMARY KEY,content TEXT NOT NULL); CREATE TABLE IF NOT EXISTS inventory (id TEXT PRIMARY KEY,name TEXT NOT NULL,quantity INTEGER NOT NULL,description TEXT NOT NULL DEFAULT ''); CREATE TABLE IF NOT EXISTS skills (id TEXT PRIMARY KEY,name TEXT NOT NULL,level INTEGER NOT NULL,experience INTEGER NOT NULL); CREATE TABLE IF NOT EXISTS goals (id TEXT PRIMARY KEY,description TEXT NOT NULL,progress INTEGER NOT NULL,target INTEGER NOT NULL,completed INTEGER NOT NULL);")?;
        let _ = self.db.execute("ALTER TABLE events ADD COLUMN participants TEXT NOT NULL DEFAULT '[]'", []);
        Ok(())
    }

    fn seed_defaults(&mut self) -> rusqlite::Result<()> {
        if self.value("name")?.is_some() { return Ok(()); }
        let tx = self.db.transaction()?;
        for (key, value) in [("name", "Aoi"), ("level", "3"), ("xp", "284"), ("next_xp", "450"), ("mood", "72"), ("energy", "61"), ("location", "图书馆"), ("weather", "小雨 · 18°C"), ("status", "正在阅读"), ("animation", "idle")] {
            tx.execute("INSERT INTO world_state(key,value) VALUES (?1,?2)", params![key, value])?;
        }
        let traits = [("好奇",78,"#f0a44b"),("善良",82,"#d66b62"),("自信",42,"#5c9a9b"),("专注",66,"#7582b6")];
        for (name, score, color) in traits { tx.execute("INSERT INTO personality_traits VALUES (?1,?2,?3)", params![name,score,color])?; }
        let npcs = [("aoi","Aoi","同学 · 朋友","A"),("yuki","Yuki","邻居 · 熟人","Y"),("ren","Ren","图书管理员","R")];
        for (id,name,role,avatar) in npcs { tx.execute("INSERT INTO npcs VALUES (?1,?2,?3,?4)", params![id,name,role,avatar])?; }
        for (id, score, stage) in [("aoi",42,"friend"),("yuki",18,"acquaintance"),("ren",9,"acquaintance")] { tx.execute("INSERT INTO relationships VALUES (?1,?2,?3)", params![id,score,stage])?; }
        tx.execute("INSERT INTO world_state(key,value) VALUES ('skills',?1)", params![r#"[{"name":"阅读","level":4,"xp":72},{"name":"绘画","level":2,"xp":38},{"name":"专注","level":3,"xp":61}]"#])?;
        tx.execute("INSERT INTO world_state(key,value) VALUES ('inventory',?1)", params![r#"[{"name":"旧书签","detail":"Aoi 送的纪念品","icon":"bookmark"},{"name":"雨伞","detail":"透明的蓝色雨伞","icon":"umbrella"},{"name":"笔记本","detail":"记录着她的想法","icon":"notebook"}]"#])?;
        tx.execute("INSERT INTO world_state(key,value) VALUES ('goals',?1)", params![r#"[{"name":"读完 5 本书","progress":3,"target":5},{"name":"学会画画","progress":42,"target":100},{"name":"成为更好的朋友","progress":68,"target":100}]"#])?;
        tx.commit()?;
        Ok(())
    }

    fn value(&self, key: &str) -> rusqlite::Result<Option<String>> { self.db.query_row("SELECT value FROM world_state WHERE key=?1", [key], |r| r.get(0)).optional() }
    fn number(&self, key: &str, default: i32) -> i32 { self.value(key).ok().flatten().and_then(|v| v.parse().ok()).unwrap_or(default) }
    fn json<T: for<'a> Deserialize<'a>>(&self, key: &str, default: T) -> T { self.value(key).ok().flatten().and_then(|v| serde_json::from_str(&v).ok()).unwrap_or(default) }
    fn set(tx: &rusqlite::Transaction<'_>, key: &str, value: impl ToString) -> rusqlite::Result<()> { tx.execute("INSERT OR REPLACE INTO world_state(key,value) VALUES (?1,?2)", params![key,value.to_string()]).map(|_| ()) }

    pub fn snapshot(&self) -> rusqlite::Result<WorldSnapshot> {
        let now = Local::now();
        let events = self.events()?;
        let important_today = events.iter().filter(|e| e.event_type == "important_event" && e.timestamp.starts_with(&now.format("%Y-%m-%d").to_string())).count() as i32;
        let traits = { let mut stmt=self.db.prepare("SELECT name,score,color FROM personality_traits ORDER BY rowid")?; let rows=stmt.query_map([], |r| Ok(Trait{name:r.get(0)?,score:r.get(1)?,color:r.get(2)?}))?; rows.collect::<Result<Vec<_>,_>>()? };
        let npcs = { let mut stmt=self.db.prepare("SELECT n.id,n.name,n.role,COALESCE(r.score,0),COALESCE(r.stage,'acquaintance'),n.avatar FROM npcs n LEFT JOIN relationships r ON r.npc_id=n.id ORDER BY n.rowid")?; let rows=stmt.query_map([], |r| Ok(Npc{id:r.get(0)?,name:r.get(1)?,role:r.get(2)?,relationship:r.get(3)?,stage:r.get(4)?,avatar:r.get(5)?}))?; rows.collect::<Result<Vec<_>,_>>()? };
        let next = self.value("next_normal_check")?.and_then(|v| v.parse().ok());
        let memories = {
            let mut stmt = self.db.prepare("SELECT summary FROM memories ORDER BY created_at DESC LIMIT 50")?;
            let rows = stmt.query_map([], |r| r.get(0))?; rows.collect::<Result<Vec<String>, _>>()?
        };
        let personality_evidence = {
            let mut stmt = self.db.prepare("SELECT trait,delta,event_id,reason FROM personality_evidence ORDER BY id DESC LIMIT 50")?;
            let rows = stmt.query_map([], |r| Ok(PersonalityEvidence { trait_name: r.get(0)?, delta: r.get(1)?, event_id: r.get(2)?, reason: r.get(3)? }))?;
            rows.collect::<Result<Vec<_>, _>>()?
        };
        Ok(WorldSnapshot { name:self.value("name")?.unwrap_or_else(||"Aoi".into()), level:self.number("level",1), xp:self.number("xp",0), next_xp:self.number("next_xp",100), mood:self.number("mood",50), energy:self.number("energy",50), location:self.value("location")?.unwrap_or_default(), weather:self.value("weather")?.unwrap_or_default(), status:self.value("status")?.unwrap_or_else(||"正在休息".into()), animation:self.value("animation")?.unwrap_or_else(||"idle".into()), traits, skills:self.json("skills", vec![]), inventory:self.json("inventory", vec![]), goals:self.json("goals", vec![]), npcs, events, world_time:now.format("%H:%M").to_string(), last_update:self.value("last_update")?.unwrap_or_else(||now.to_rfc3339()), important_today, next_normal_check:next, memory_context:self.memory_context(), memories, personality_evidence })
    }

    fn events(&self) -> rusqlite::Result<Vec<EventRecord>> { let mut stmt=self.db.prepare("SELECT id,timestamp,type,summary,importance,location,participants,causes FROM events ORDER BY timestamp DESC")?; let rows=stmt.query_map([], |r| Ok(EventRecord{id:r.get(0)?,timestamp:r.get(1)?,event_type:r.get(2)?,summary:r.get(3)?,importance:r.get(4)?,location:r.get(5)?,participants:serde_json::from_str(&r.get::<_,String>(6)?).unwrap_or_default(),causes:serde_json::from_str(&r.get::<_,String>(7)?).unwrap_or_default()}))?; rows.collect()
    }

    fn project_root(&self) -> PathBuf {
        if let Some(path) = std::env::var_os("AI_WORLD_PROJECT").map(PathBuf::from) {
            return path;
        }
        workspace_root()
            .or_else(|| self.root.parent().map(PathBuf::from))
            .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")))
    }

    pub fn memory_context(&self) -> String {
        let root = self.project_root();
        let mut files = vec![root.join("world/rules.md"),root.join("character/character.md"),root.join("character/personality.md"),root.join("character/relationships.md")];
        if let Ok(entries)=fs::read_dir(root.join("character/important_people")) { files.extend(entries.flatten().map(|e|e.path()).filter(|p|p.extension().and_then(|x|x.to_str())==Some("md"))); }
        files.sort(); files.into_iter().filter_map(|path| fs::read_to_string(&path).ok().map(|body| format!("\n## {}\n{}", path.display(), body))).collect()
    }

    pub fn scheduler_tick(&mut self) -> rusqlite::Result<Option<String>> {
        let now = Local::now().timestamp();
        let normal_due = self.value("next_normal_check")?.and_then(|v| v.parse::<i64>().ok()).map(|next| now >= next).unwrap_or(true);
        if normal_due {
            return Ok(Some("normal".into()));
        }
        let important_due = self.value("next_important_check")?.and_then(|v| v.parse::<i64>().ok()).map(|next| now >= next).unwrap_or(true);
        if !important_due {
            return Ok(None);
        }
        let last_important = self.value("last_important")?.and_then(|v| v.parse::<i64>().ok()).unwrap_or(0);
        let hours_since = ((now - last_important).max(0) as f32 / 3600.0).min(24.0);
        let recent_count = self.events()?.iter().filter(|event| event.timestamp.parse::<DateTime<Local>>().ok().map(|time| (Local::now() - time).num_hours() < 24).unwrap_or(false)).count() as u32;
        let important_today = self.events()?.iter().filter(|event| event.event_type == "important_event" && event.timestamp.starts_with(&Local::now().format("%Y-%m-%d").to_string())).count() as u32;
        let target = self.number("important_target_per_day", 2) as f32;
        let random_factor = ((now.rem_euclid(1000)) as f32 / 1000.0).clamp(0.0, 1.0);
        let context = crate::scheduler::WindowContext { hours_since_important: hours_since, important_today, recent_event_count: recent_count, goal_pressure: 0.4, relationship_opportunity: 0.4, random_factor };
        let selected = crate::scheduler::should_schedule(context, target);
        let tx = self.db.transaction()?;
        Self::set(&tx, "next_important_check", now + 4 * 3600)?;
        tx.commit()?;
        Ok(selected.then_some("important".into()))
    }

    pub fn reset(&mut self) -> rusqlite::Result<()> { self.db.execute_batch("DELETE FROM world_state; DELETE FROM events; DELETE FROM personality_evidence; DELETE FROM relationships; DELETE FROM shared_experiences; DELETE FROM memories; DELETE FROM inventory; DELETE FROM skills; DELETE FROM goals; DELETE FROM personality_traits; DELETE FROM npcs; DELETE FROM characters; DELETE FROM important_people;")?; self.seed_defaults() }

    pub fn apply(&mut self, p: EventProposal) -> rusqlite::Result<WorldSnapshot> {
        if p.event_type == "no_event" {
            return Err(rusqlite::Error::InvalidParameterName("silent events are disabled".into()));
        }
        if !EVENT_TYPES.contains(&p.event_type.as_str()) { return Err(rusqlite::Error::InvalidParameterName("invalid event type".into())); }
        if p.summary.trim().is_empty() || p.summary.chars().count()>160 { return Err(rusqlite::Error::InvalidParameterName("summary must be 1-160 characters".into())); }
        let xp=self.number("xp",0); let mut level=self.number("level",1); let mut next=self.number("next_xp",100); let mood=(self.number("mood",50)+p.effects.mood).clamp(0,100); let energy=(self.number("energy",50)+p.effects.energy).clamp(0,100); let new_xp_delta=p.xp_delta().clamp(0,50); let mut total=xp+new_xp_delta;
        while total>=next { total-=next; level+=1; next=(next as f32*1.35).round() as i32; }
        let now:DateTime<Local>=Local::now(); let id=format!("event-{}",now.timestamp_millis()); let rel=p.relationship().map(|r|(r.target.clone(),r.delta.clamp(-5,5)));
        let item_effect=p.effects.item.clone(); let skill_effect=p.effects.skill.clone(); let goal_effect=p.effects.goal.clone();
        let personality_signal=p.effects.personality_signal.clone();
        let mut items: Vec<InventoryItem> = self.json("inventory", vec![]);
        let mut skills: Vec<Skill> = self.json("skills", vec![]);
        let mut goals: Vec<Goal> = self.json("goals", vec![]);
        let tx=self.db.transaction()?;
        tx.execute("INSERT INTO events(id,timestamp,type,summary,importance,location,causes,participants) VALUES (?1,?2,?3,?4,?5,?6,?7,?8)",params![&id,now.to_rfc3339(),&p.event_type,&p.summary,p.importance.clamp(0.0,1.0),&p.location,serde_json::to_string(&p.causes).unwrap(),serde_json::to_string(&p.participants).unwrap()])?;
        Self::set(&tx,"xp",total)?; Self::set(&tx,"level",level)?; Self::set(&tx,"next_xp",next)?; Self::set(&tx,"mood",mood)?; Self::set(&tx,"energy",energy)?; Self::set(&tx,"location",&p.location)?; Self::set(&tx,"last_update",now.to_rfc3339())?; Self::set(&tx,"next_normal_check",now.timestamp()+1800)?;
        if let Some((target,delta))=rel {
            let score: i32 = tx.query_row("SELECT score FROM relationships WHERE npc_id=?1", params![target], |row| row.get(0)).optional()?.unwrap_or(0);
            let updated = (score + delta).clamp(0, 100);
            let stage = relationship_stage(updated);
            tx.execute("INSERT INTO relationships(npc_id,score,stage) VALUES (?1,?2,?3) ON CONFLICT(npc_id) DO UPDATE SET score=excluded.score,stage=excluded.stage",params![target,updated,stage])?;
        }
        if let Some(item)=item_effect {
            if item.quantity > 0 {
                if let Some(existing)=items.iter_mut().find(|entry| entry.name == item.name) { existing.detail=item.description.clone(); }
                else { items.push(InventoryItem { name:item.name, detail:item.description, icon:item.id }); }
                Self::set(&tx,"inventory",serde_json::to_string(&items).unwrap())?;
            }
        }
        if let Some(skill)=skill_effect {
            if skill.experience > 0 {
                if let Some(existing)=skills.iter_mut().find(|entry| entry.name == skill.name) { existing.xp += skill.experience; while existing.xp >= 100 { existing.level += 1; existing.xp -= 100; } }
                else { skills.push(Skill { name:skill.name, level:1, xp:skill.experience.min(99) }); }
                Self::set(&tx,"skills",serde_json::to_string(&skills).unwrap())?;
            }
        }
        if let Some(goal)=goal_effect {
            if goal.progress > 0 { if let Some(existing)=goals.iter_mut().find(|entry| entry.name == goal.id) { existing.progress=(existing.progress+goal.progress).min(existing.target); } Self::set(&tx,"goals",serde_json::to_string(&goals).unwrap())?; }
        }
        if let Some(signal)=personality_signal { if (p.event_type=="important_event" || p.event_type=="milestone_event" || p.event_type=="relationship_event") && !signal.trait_name.trim().is_empty() { let delta=signal.delta.clamp(-3,3); tx.execute("UPDATE personality_traits SET score=MAX(0,MIN(100,score+?1)) WHERE name=?2",params![delta,signal.trait_name])?; tx.execute("INSERT INTO personality_evidence(trait,delta,event_id,reason) VALUES (?1,?2,?3,?4)",params![signal.trait_name,delta,id,signal.reason])?; } }
        if p.memory { tx.execute("INSERT INTO memories(id,event_id,summary,created_at) VALUES (?1,?2,?3,?4)",params![&id,&id,&p.summary,now.to_rfc3339()])?; }
        if p.event_type == "important_event" || p.event_type == "milestone_event" || p.event_type == "relationship_event" {
            if p.participants.len() > 1 {
                tx.execute("INSERT INTO shared_experiences(event_ids,summary) VALUES (?1,?2)", params![serde_json::to_string(&[id.clone()]).unwrap(), &p.summary])?;
            }
        }
        tx.commit()?;
        self.sync_markdown(&id)?;
        self.snapshot()
    }

    fn sync_from_markdown(&self) -> rusqlite::Result<()> {
        let root = self.project_root();
        let personality = root.join("character/personality.md");
        if let Ok(content) = fs::read_to_string(personality) {
            let tx = self.db.unchecked_transaction()?;
            for line in content.lines() {
                if let Some((name, score)) = line.trim().strip_prefix("- ").and_then(|line| line.split_once(':')) {
                    if let Some((value, _)) = score.trim().split_once('/') {
                        if let Ok(score) = value.trim().parse::<i32>() {
                            let _ = tx.execute("UPDATE personality_traits SET score=?1 WHERE lower(name)=lower(?2)", params![score.clamp(0, 100), name.trim()]);
                        }
                    }
                }
            }
            tx.commit()?;
        }
        Ok(())
    }

    fn sync_markdown(&self, event_id: &str) -> rusqlite::Result<()> {
        let root = self.project_root();
        let character_dir = root.join("character");
        if !character_dir.exists() { return Ok(()); }
        let traits = {
            let mut stmt = self.db.prepare("SELECT name,score FROM personality_traits ORDER BY rowid")?;
            let rows = stmt.query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, i32>(1)?)))?; rows.collect::<Result<Vec<_>, _>>()?
        };
        let evidence = {
            let mut stmt = self.db.prepare("SELECT event_id,reason,trait,delta FROM personality_evidence ORDER BY id DESC LIMIT 20")?;
            let rows = stmt.query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?, r.get::<_, String>(2)?, r.get::<_, i32>(3)?)))?; rows.collect::<Result<Vec<_>, _>>()?
        };
        let mut personality = String::from("# Personality Development\n\n## Trait Scores\n");
        for (name, score) in traits { personality.push_str(&format!("- {}: {}/100\n", name, score)); }
        personality.push_str("\n## Personality Evidence\n");
        for (id, reason, trait_name, delta) in evidence { personality.push_str(&format!("- {}: {}; {} {:+}.\n", id, reason, trait_name, delta)); }
        personality.push_str(&format!("\n## Last Applied Event\n- {}\n", event_id));
        fs::write(character_dir.join("personality.md"), personality).map_err(|_| rusqlite::Error::ToSqlConversionFailure(Box::new(std::io::Error::other("failed to write personality.md"))))?;

        let npcs = {
            let mut stmt = self.db.prepare("SELECT n.name,COALESCE(r.score,0),COALESCE(r.stage,'acquaintance') FROM npcs n LEFT JOIN relationships r ON r.npc_id=n.id ORDER BY n.rowid")?;
            let rows = stmt.query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, i32>(1)?, r.get::<_, String>(2)?)))?; rows.collect::<Result<Vec<_>, _>>()?
        };
        let mut relationships = String::from("# Relationships\n\n");
        for (name, score, stage) in npcs { relationships.push_str(&format!("## {}\n- Stage: {}\n- Score: {}/100\n\n", name, stage, score)); }
        fs::write(character_dir.join("relationships.md"), relationships).map_err(|_| rusqlite::Error::ToSqlConversionFailure(Box::new(std::io::Error::other("failed to write relationships.md"))))?;
        Ok(())
    }
}

fn relationship_stage(score: i32) -> &'static str {
    match score {
        0..=19 => "acquaintance",
        20..=49 => "friend",
        50..=79 => "close_friend",
        _ => "trusted",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine() -> Engine {
        let db = Connection::open_in_memory().unwrap();
        let mut engine = Engine { db, root: PathBuf::from(".") };
        engine.migrate().unwrap();
        engine.seed_defaults().unwrap();
        engine
    }

    #[test]
    fn default_proposal_is_no_event() {
        assert_eq!(EventProposal::default().event_type, "no_event");
    }

    #[test]
    fn silent_event_is_rejected() {
        let mut engine = engine();
        assert!(engine.apply(EventProposal::default()).is_err());
    }

    #[test]
    fn parses_snake_case_event_type_from_llm() {
        let proposal: EventProposal = serde_json::from_str(r#"{"event_type":"discovery_event","summary":"发现一封信","importance":0.4,"location":"图书馆","effects":{},"participants":[],"causes":[],"memory":false}"#).unwrap();
        assert_eq!(proposal.event_type, "discovery_event");
    }

    #[test]
    fn apply_persists_event_and_clamps_state() {
        let mut engine = engine();
        let snapshot = engine.apply(EventProposal {
            event_type: "normal_event".into(), summary: "测试事件".into(), importance: 2.0,
            location: "房间".into(), effects: EventEffects { mood: 50, energy: -100, xp: 50, ..Default::default() },
            participants: vec!["main".into()], causes: vec!["event-old".into()], memory: true,
        }).unwrap();
        assert_eq!(snapshot.mood, 100);
        assert_eq!(snapshot.energy, 0);
        assert_eq!(snapshot.events[0].participants, vec!["main"]);
        assert_eq!(snapshot.events[0].causes, vec!["event-old"]);
    }

    #[test]
    fn relationship_updates_stage() {
        let mut engine = engine();
        let snapshot = engine.apply(EventProposal {
            event_type: "relationship_event".into(), summary: "关系变化".into(), importance: 0.5,
            location: "图书馆".into(), effects: EventEffects { relationship: Some(RelationshipEffect { target: "yuki".into(), delta: 5 }), ..Default::default() },
            participants: vec!["main".into(), "yuki".into()], causes: vec![], memory: false,
        }).unwrap();
        assert_eq!(snapshot.npcs.iter().find(|npc| npc.id == "yuki").unwrap().relationship, 23);
        assert_eq!(snapshot.npcs.iter().find(|npc| npc.id == "yuki").unwrap().stage, "friend");
    }
}

fn workspace_root() -> Option<PathBuf> {
    let current = std::env::current_dir().ok()?;
    if current.join("require.md").exists() {
        return Some(current);
    }
    current.parent().filter(|parent| parent.join("require.md").exists()).map(PathBuf::from)
}

fn dirs_path() -> PathBuf {
    std::env::var_os("AI_WORLD_DATA")
        .map(PathBuf::from)
        .or_else(|| workspace_root().map(|root| root.join(".aoi-data")))
        .unwrap_or_else(|| std::env::temp_dir().join("aoi-world-data"))
}
