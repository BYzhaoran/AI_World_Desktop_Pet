use chrono::{DateTime, Datelike, Local, Timelike};
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
    pub health: i32,
    pub intelligence: f32,
    pub friendship: f32,
    pub curiosity: f32,
    pub creativity: f32,
    pub courage: f32,
    pub exploration: i32,
    pub money: i32,
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
    #[serde(default)]
    pub relation: Option<String>,
    #[serde(default, alias = "thread_id")]
    pub thread_id: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default, alias = "estimated_duration")]
    pub estimated_duration: Option<i32>,
    #[serde(default)]
    pub progress: Option<ProgressUpdate>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgressUpdate {
    pub summary: String,
    pub progress: f32,
    pub state: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventThread {
    pub id: String,
    pub title: String,
    pub summary: String,
    #[serde(rename = "type")]
    pub event_type: String,
    pub start_time: String,
    pub last_update_time: String,
    pub end_time: Option<String>,
    pub estimated_duration: i32,
    pub actual_duration: Option<i32>,
    pub status: String,
    pub progress: f32,
    pub importance: f32,
    pub location: String,
    pub participants: Vec<String>,
    pub updates: Vec<EventProgress>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventProgress {
    pub id: String,
    pub thread_id: String,
    pub timestamp: String,
    pub summary: String,
    pub progress: f32,
    pub state: String,
    pub effects: Option<EventEffects>,
}

impl Default for EventProposal {
    fn default() -> Self { Self::no_event() }
}

impl EventProposal {
    pub fn no_event() -> Self {
        Self { event_type: "no_event".into(), summary: String::new(), importance: 0.0,
            location: String::new(), effects: EventEffects::default(), participants: vec![], causes: vec![], memory: false,
            relation: Some("new".into()), thread_id: None, title: None, estimated_duration: None, progress: None }
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
    pub mood: i32, pub energy: i32, pub health: i32, pub intelligence: f32,
    pub friendship: f32, pub curiosity: f32, pub creativity: f32, pub courage: f32, pub money: i32,
    pub location: String, pub weather: String,
    pub status: String, pub animation: String,
    pub traits: Vec<Trait>, pub skills: Vec<Skill>, pub inventory: Vec<InventoryItem>,
    pub goals: Vec<Goal>, pub npcs: Vec<Npc>, pub known_locations: Vec<Location>,
    pub events: Vec<EventRecord>,
    pub event_threads: Vec<EventThread>,
    pub world_time: String, pub last_update: String, pub important_today: i32,
    pub next_normal_check: Option<i64>, pub memory_context: String,
    pub memories: Vec<String>, pub personality_evidence: Vec<PersonalityEvidence>,
    pub day_count: i32, pub total_play_time: i64, pub current_behavior: String,
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
pub struct Npc {
    pub id: String, pub name: String, pub role: String, pub relationship: i32,
    pub stage: String, pub avatar: String, pub personality: String,
    pub favorite_item: String, pub home_location: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location { pub name: String, pub description: String, pub exploration: i32, pub rarity: String }
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
        self.db.execute_batch("CREATE TABLE IF NOT EXISTS world_state (key TEXT PRIMARY KEY,value TEXT NOT NULL); CREATE TABLE IF NOT EXISTS characters (id TEXT PRIMARY KEY,name TEXT NOT NULL,level INTEGER NOT NULL,xp INTEGER NOT NULL); CREATE TABLE IF NOT EXISTS personality_traits (name TEXT PRIMARY KEY,score INTEGER NOT NULL,color TEXT NOT NULL); CREATE TABLE IF NOT EXISTS events (id TEXT PRIMARY KEY,timestamp TEXT NOT NULL,type TEXT NOT NULL,summary TEXT NOT NULL,importance REAL NOT NULL,location TEXT NOT NULL,causes TEXT NOT NULL,participants TEXT NOT NULL DEFAULT '[]'); CREATE TABLE IF NOT EXISTS event_threads (id TEXT PRIMARY KEY,title TEXT NOT NULL,summary TEXT NOT NULL,type TEXT NOT NULL,start_time TEXT NOT NULL,last_update_time TEXT NOT NULL,end_time TEXT,estimated_duration INTEGER NOT NULL,actual_duration INTEGER,status TEXT NOT NULL,progress REAL NOT NULL,importance REAL NOT NULL,location TEXT NOT NULL,participants TEXT NOT NULL DEFAULT '[]'); CREATE TABLE IF NOT EXISTS event_progress (id TEXT PRIMARY KEY,thread_id TEXT NOT NULL,timestamp TEXT NOT NULL,summary TEXT NOT NULL,progress REAL NOT NULL,state TEXT NOT NULL,effects TEXT,FOREIGN KEY(thread_id) REFERENCES event_threads(id)); CREATE TABLE IF NOT EXISTS personality_evidence (id INTEGER PRIMARY KEY,trait TEXT NOT NULL,delta INTEGER NOT NULL,event_id TEXT NOT NULL,reason TEXT NOT NULL); CREATE TABLE IF NOT EXISTS relationships (npc_id TEXT PRIMARY KEY,score INTEGER NOT NULL,stage TEXT NOT NULL); CREATE TABLE IF NOT EXISTS shared_experiences (id INTEGER PRIMARY KEY,event_ids TEXT NOT NULL,summary TEXT NOT NULL); CREATE TABLE IF NOT EXISTS memories (id TEXT PRIMARY KEY,event_id TEXT NOT NULL,summary TEXT NOT NULL,created_at TEXT NOT NULL); CREATE TABLE IF NOT EXISTS npcs (id TEXT PRIMARY KEY,name TEXT NOT NULL,role TEXT NOT NULL,avatar TEXT NOT NULL); CREATE TABLE IF NOT EXISTS important_people (id TEXT PRIMARY KEY,content TEXT NOT NULL); CREATE TABLE IF NOT EXISTS inventory (id TEXT PRIMARY KEY,name TEXT NOT NULL,quantity INTEGER NOT NULL,description TEXT NOT NULL DEFAULT ''); CREATE TABLE IF NOT EXISTS skills (id TEXT PRIMARY KEY,name TEXT NOT NULL,level INTEGER NOT NULL,experience INTEGER NOT NULL); CREATE TABLE IF NOT EXISTS goals (id TEXT PRIMARY KEY,description TEXT NOT NULL,progress INTEGER NOT NULL,target INTEGER NOT NULL,completed INTEGER NOT NULL);")?;
        let _ = self.db.execute("ALTER TABLE events ADD COLUMN participants TEXT NOT NULL DEFAULT '[]'", []);
        let _ = self.db.execute("ALTER TABLE npcs ADD COLUMN personality TEXT NOT NULL DEFAULT ''", []);
        let _ = self.db.execute("ALTER TABLE npcs ADD COLUMN favorite_item TEXT NOT NULL DEFAULT ''", []);
        let _ = self.db.execute("ALTER TABLE npcs ADD COLUMN home_location TEXT NOT NULL DEFAULT ''", []);
        self.db.execute("DELETE FROM relationships WHERE npc_id IN ('aoi','yuki','ren')", [])?;
        self.db.execute("DELETE FROM npcs WHERE id IN ('aoi','yuki','ren')", [])?;
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
        tx.execute("INSERT INTO world_state(key,value) VALUES ('skills',?1)", params![r#"[{"name":"阅读","level":4,"xp":72},{"name":"绘画","level":2,"xp":38},{"name":"专注","level":3,"xp":61}]"#])?;
        tx.execute("INSERT INTO world_state(key,value) VALUES ('inventory',?1)", params![r#"[{"name":"旧书签","detail":"Aoi 送的纪念品","icon":"bookmark"},{"name":"雨伞","detail":"透明的蓝色雨伞","icon":"umbrella"},{"name":"笔记本","detail":"记录着她的想法","icon":"notebook"}]"#])?;
        tx.execute("INSERT INTO world_state(key,value) VALUES ('goals',?1)", params![r#"[{"name":"读完 5 本书","progress":3,"target":5},{"name":"学会画画","progress":42,"target":100},{"name":"成为更好的朋友","progress":68,"target":100}]"#])?;
        tx.execute("INSERT OR REPLACE INTO world_state(key,value) VALUES ('known_locations',?1)", params![r#"[{"name":"Home","description":"A quiet place to rest.","exploration":35,"rarity":"common"}]"#])?;
        tx.commit()?;
        Ok(())
    }

    fn value(&self, key: &str) -> rusqlite::Result<Option<String>> { self.db.query_row("SELECT value FROM world_state WHERE key=?1", [key], |r| r.get(0)).optional() }
    fn number(&self, key: &str, default: i32) -> i32 { self.value(key).ok().flatten().and_then(|v| v.parse().ok()).unwrap_or(default) }
    fn decimal(&self, key: &str, default: f32) -> f32 { self.value(key).ok().flatten().and_then(|v| v.parse().ok()).unwrap_or(default) }
    fn json<T: for<'a> Deserialize<'a>>(&self, key: &str, default: T) -> T { self.value(key).ok().flatten().and_then(|v| serde_json::from_str(&v).ok()).unwrap_or(default) }
    fn set(tx: &rusqlite::Transaction<'_>, key: &str, value: impl ToString) -> rusqlite::Result<()> { tx.execute("INSERT OR REPLACE INTO world_state(key,value) VALUES (?1,?2)", params![key,value.to_string()]).map(|_| ()) }
    pub fn set_rest_hours(&mut self, start: i32, end: i32) -> rusqlite::Result<()> {
        let tx = self.db.transaction()?;
        Self::set(&tx, "rest_start", start.clamp(0, 23))?;
        Self::set(&tx, "rest_end", end.clamp(0, 23))?;
        tx.commit()
    }
    pub fn in_rest_period(&self, now: DateTime<Local>) -> bool {
        let start = self.number("rest_start", 22).clamp(0, 23) as u32;
        let end = self.number("rest_end", 8).clamp(0, 23) as u32;
        let hour = now.hour();
        if start == end { true } else if start < end { hour >= start && hour < end } else { hour >= start || hour < end }
    }

    pub fn snapshot(&self) -> rusqlite::Result<WorldSnapshot> {
        let now = Local::now();
        let events = self.events()?;
        let event_threads = self.event_threads()?;
        let important_today = events.iter().filter(|e| e.event_type == "important_event" && e.timestamp.starts_with(&now.format("%Y-%m-%d").to_string())).count() as i32;
        let traits = { let mut stmt=self.db.prepare("SELECT name,score,color FROM personality_traits ORDER BY rowid")?; let rows=stmt.query_map([], |r| Ok(Trait{name:r.get(0)?,score:r.get(1)?,color:r.get(2)?}))?; rows.collect::<Result<Vec<_>,_>>()? };
        let npcs = { let mut stmt=self.db.prepare("SELECT n.id,n.name,n.role,COALESCE(r.score,0),COALESCE(r.stage,'acquaintance'),n.avatar,n.personality,n.favorite_item,n.home_location FROM npcs n LEFT JOIN relationships r ON r.npc_id=n.id ORDER BY n.rowid")?; let rows=stmt.query_map([], |r| Ok(Npc{id:r.get(0)?,name:r.get(1)?,role:r.get(2)?,relationship:r.get(3)?,stage:r.get(4)?,avatar:r.get(5)?,personality:r.get(6)?,favorite_item:r.get(7)?,home_location:r.get(8)?}))?; rows.collect::<Result<Vec<_>,_>>()? };
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
        Ok(WorldSnapshot { name:self.value("name")?.unwrap_or_else(||"Aoi".into()), level:self.number("level",1), xp:self.number("xp",0), next_xp:self.number("next_xp",100), mood:self.number("mood",50), energy:self.number("energy",50), health:self.number("health",100), intelligence:self.decimal("intelligence",50.0), friendship:self.decimal("friendship",0.0), curiosity:self.decimal("curiosity",50.0), creativity:self.decimal("creativity",50.0), courage:self.decimal("courage",50.0), money:self.number("money",0), location:self.value("location")?.unwrap_or_default(), weather:self.value("weather")?.unwrap_or_default(), status:self.value("status")?.unwrap_or_else(||"正在休息".into()), animation:self.value("animation")?.unwrap_or_else(||"idle".into()), traits, skills:self.json("skills", vec![]), inventory:self.json("inventory", vec![]), goals:self.json("goals", vec![]), npcs, known_locations:self.json("known_locations", vec![]), events, event_threads, world_time:now.format("%H:%M").to_string(), last_update:self.value("last_update")?.unwrap_or_else(||now.to_rfc3339()), important_today, next_normal_check:next, memory_context:self.memory_context(), memories, personality_evidence, day_count:self.number("day_count",1), total_play_time:self.value("total_play_time")?.and_then(|v|v.parse().ok()).unwrap_or(0), current_behavior:self.value("current_behavior")?.unwrap_or_else(||"idle".into()) })
    }

    fn events(&self) -> rusqlite::Result<Vec<EventRecord>> { let mut stmt=self.db.prepare("SELECT id,timestamp,type,summary,importance,location,participants,causes FROM events ORDER BY timestamp DESC")?; let rows=stmt.query_map([], |r| Ok(EventRecord{id:r.get(0)?,timestamp:r.get(1)?,event_type:r.get(2)?,summary:r.get(3)?,importance:r.get(4)?,location:r.get(5)?,participants:serde_json::from_str(&r.get::<_,String>(6)?).unwrap_or_default(),causes:serde_json::from_str(&r.get::<_,String>(7)?).unwrap_or_default()}))?; rows.collect()
    }

    fn event_threads(&self) -> rusqlite::Result<Vec<EventThread>> {
        let mut stmt = self.db.prepare("SELECT id,title,summary,type,start_time,last_update_time,end_time,estimated_duration,actual_duration,status,progress,importance,location,participants FROM event_threads ORDER BY start_time DESC")?;
        let rows = stmt.query_map([], |r| {
            let id: String = r.get(0)?;
            let mut progress_stmt = self.db.prepare("SELECT id,thread_id,timestamp,summary,progress,state,effects FROM event_progress WHERE thread_id=?1 ORDER BY timestamp ASC")?;
            let updates = progress_stmt.query_map([&id], |p| {
                let effects = p.get::<_, Option<String>>(6)?.and_then(|value| serde_json::from_str(&value).ok());
                Ok(EventProgress { id:p.get(0)?, thread_id:p.get(1)?, timestamp:p.get(2)?, summary:p.get(3)?, progress:p.get(4)?, state:p.get(5)?, effects })
            })?.collect::<Result<Vec<_>, _>>()?;
            Ok(EventThread {
                id, title:r.get(1)?, summary:r.get(2)?, event_type:r.get(3)?,
                start_time:r.get(4)?, last_update_time:r.get(5)?, end_time:r.get(6)?,
                estimated_duration:r.get(7)?, actual_duration:r.get(8)?, status:r.get(9)?,
                progress:r.get(10)?, importance:r.get(11)?, location:r.get(12)?,
                participants:serde_json::from_str(&r.get::<_, String>(13)?).unwrap_or_default(),
                updates,
            })
        })?;
        rows.collect()
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

    pub fn location_change_due(&self) -> bool {
        self.location_change_due_at(Local::now())
    }

    fn location_change_due_at(&self, now: DateTime<Local>) -> bool {
        if self.in_rest_period(now) {
            return false;
        }
        let now = now.timestamp();
        let last = self.value("last_location_change").ok().flatten()
            .and_then(|value| value.parse::<i64>().ok()).unwrap_or(now);
        now - last >= 4 * 3600
    }

    fn enforce_location_schedule(&mut self, now: DateTime<Local>) -> rusqlite::Result<bool> {
        let current = self.value("location")?.unwrap_or_else(|| "家".into());
        let target = if self.in_rest_period(now) {
            if current == "家" { return Ok(false); }
            Some("家".to_string())
        } else if self.location_change_due_at(now) {
            let locations: Vec<Location> = self.json("known_locations", vec![]);
            let alternatives: Vec<String> = locations.into_iter()
                .map(|location| location.name)
                .filter(|name| !name.is_empty() && name != &current)
                .collect();
            if alternatives.is_empty() {
                None
            } else {
                Some(alternatives[(now.timestamp().unsigned_abs() as usize) % alternatives.len()].clone())
            }
        } else {
            None
        };
        let Some(target) = target else { return Ok(false); };
        let tx = self.db.transaction()?;
        Self::set(&tx, "location", &target)?;
        Self::set(&tx, "last_location_change", now.timestamp())?;
        tx.commit()?;
        eprintln!("[world] scheduled location change: {} -> {}", current, target);
        Ok(true)
    }

    pub fn scheduler_tick(&mut self) -> rusqlite::Result<Option<String>> {
        let now = Local::now().timestamp();
        let Some(last_value) = self.value("last_update")? else {
            let tx = self.db.transaction()?;
            Self::set(&tx, "last_update", Local::now().to_rfc3339())?;
            Self::set(&tx, "next_normal_check", now + 10 * 60)?;
            Self::set(&tx, "birth_day", now / 86400)?;
            tx.commit()?;
            return Ok(None);
        };
        let last = DateTime::parse_from_rfc3339(&last_value).ok().map(|v| v.timestamp()).unwrap_or(now);
        let elapsed = (now - last).clamp(0, 24 * 3600);
        if elapsed > 0 {
            self.simulate_elapsed(now, elapsed)?;
        }
        self.enforce_location_schedule(Local::now())?;
        let now = Local::now().timestamp();
        if let Some(kind) = self.run_time_constraints(Local::now())? {
            return Ok(Some(kind));
        }
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
        let selected = important_today < 3 && self.important_day_allowed(Local::now()) && crate::scheduler::should_schedule(context, target);
        let tx = self.db.transaction()?;
        Self::set(&tx, "next_important_check", now + 4 * 3600)?;
        tx.commit()?;
        Ok(selected.then_some("important".into()))
    }

    fn run_time_constraints(&mut self, now: DateTime<Local>) -> rusqlite::Result<Option<String>> {
        let hour = now.hour();
        let minute = now.minute();
        let in_window = |start: u32, end: u32| {
            let current = hour * 60 + minute;
            current >= start && current <= end
        };
        let date = if hour < 8 { (now - chrono::Duration::days(1)).format("%Y-%m-%d") } else { now.format("%Y-%m-%d") };
        let rest_start = (self.number("rest_start", 22).clamp(0, 23) as u32) * 60;
        let rest_end = (self.number("rest_end", 8).clamp(0, 23) as u32) * 60;
        let rules = [
            ("breakfast", 7 * 60, 10 * 60, "吃了早饭，开始准备今天的生活。", "activity_event"),
            ("lunch", 11 * 60 + 30, 14 * 60, "吃了一顿午饭，能量恢复了一些。", "activity_event"),
            ("dinner", 17 * 60 + 30, 21 * 60, "吃了晚饭，回顾今天发生的事情。", "activity_event"),
            ("sleep", 22 * 60, 8 * 60, "进入睡眠，安静地恢复能量。", "activity_event"),
            ("special_activity", 19 * 60, 22 * 60, "参加了一次夜间特别活动，发现了新的线索。", "discovery_event"),
        ];
        for (key, start, end, summary, event_type) in rules {
            let matches = if key == "sleep" {
                let current = hour * 60 + minute;
                if rest_start == rest_end { true } else if rest_start < rest_end {
                    current >= rest_start && current < rest_end
                } else {
                    current >= rest_start || current < rest_end
                }
            } else { in_window(start, end) };
            if !matches { continue; }
            let marker = format!("time_event_{}_{}", date, key);
            if self.value(&marker)?.is_some() { continue; }
            let id = format!("timed-{}-{}", key, now.timestamp());
            let location = self.value("location")?.unwrap_or_default();
            let energy = (self.number("energy", 50) + 8).min(100);
            let tx = self.db.transaction()?;
            tx.execute("INSERT INTO events(id,timestamp,type,summary,importance,location,causes,participants) VALUES (?1,?2,?3,?4,?5,?6,'[]','[\"main\"]')",
                params![id, now.to_rfc3339(), event_type, summary, if key == "special_activity" { 0.55 } else { 0.18 }, location])?;
            Self::set(&tx, &marker, "1")?;
            if key == "sleep" { Self::set(&tx, "energy", energy)?; }
            tx.commit()?;
            return Ok(Some(key.into()));
        }
        Ok(None)
    }

    fn important_day_allowed(&self, now: DateTime<Local>) -> bool {
        if let Some(days) = self.value("important_days").ok().flatten().and_then(|v| serde_json::from_str::<Vec<u32>>(&v).ok()) {
            if !days.is_empty() && !days.contains(&now.weekday().number_from_monday()) { return false; }
        }
        if let Some(dates) = self.value("important_dates").ok().flatten().and_then(|v| serde_json::from_str::<Vec<String>>(&v).ok()) {
            if !dates.is_empty() && !dates.contains(&now.format("%Y-%m-%d").to_string()) { return false; }
        }
        true
    }

    fn simulate_elapsed(&mut self, now: i64, elapsed: i64) -> rusqlite::Result<()> {
        let offline = elapsed > 15 * 60;
        let minutes = (elapsed / 60).min(24 * 60);
        if minutes == 0 { return Ok(()); }
        let mut energy = self.number("energy", 50) as f32;
        let mut mood = self.number("mood", 50) as f32;
        let mut health = self.number("health", 100) as f32;
        let mut curiosity = self.decimal("curiosity", 50.0);
        let intelligence = self.decimal("intelligence", 50.0);
        let mut friendship = self.decimal("friendship", 0.0);
        let creativity = self.decimal("creativity", 50.0);
        let courage = self.decimal("courage", 50.0);
        let mut xp = self.number("xp", 0);
        let mut level = self.number("level", 1);
        let mut next_xp = self.number("next_xp", 100);
        let mut locations: Vec<Location> = self.json("known_locations", vec![]);
        let mut behavior = self.value("current_behavior")?.unwrap_or_else(|| "idle".into());
        let mut behavior_until = self.value("behavior_until")?.and_then(|v| v.parse::<i64>().ok()).unwrap_or(0);
        let mut event_count = 0;
        let seed = now as u64 ^ elapsed as u64;

        for minute in 0..minutes {
            let cursor = now - (minutes - minute) * 60;
            let in_rest_period = chrono::DateTime::<chrono::Utc>::from_timestamp(cursor, 0)
                .map(|time| self.in_rest_period(time.with_timezone(&Local)))
                .unwrap_or(false);
            if offline {
                behavior = if in_rest_period && energy < 35.0 { "sleep".into() } else { "rest".into() };
                energy += if behavior == "sleep" { 0.30 } else { 0.04 };
                mood += if behavior == "sleep" { 0.04 } else { 0.0 };
            } else {
                if energy < 5.0 {
                    behavior = if in_rest_period { "sleep".into() } else { "rest".into() };
                    behavior_until = cursor + 180;
                } else if cursor >= behavior_until {
                    let roll = unit(seed.wrapping_add(minute as u64 * 7919));
                    behavior = choose_behavior(roll, energy, mood, intelligence, friendship, curiosity, creativity, courage);
                    if behavior == "sleep" && !in_rest_period {
                        behavior = "rest".into();
                    }
                    behavior_until = cursor + 30 + ((unit(seed + minute as u64 * 104729) * 150.0) as i64);
                }
                let cost = match behavior.as_str() {
                    "sleep" => -0.30,
                    "play" => -0.08,
                    "explore" => -0.10,
                    "work" => -0.12,
                    "run" => -0.18,
                    "social" => -0.06,
                    "observe" => -0.03,
                    _ => -0.02,
                };
                energy += cost;
                if behavior == "play" { mood += 0.08 + creativity * 0.0005; }
                if behavior == "explore" { curiosity += 0.02; }
                if behavior == "work" { xp += (intelligence / 100.0) as i32; }
                if behavior == "social" { mood += friendship * 0.0005; }
                if behavior == "explore" {
                    if let Some(location) = locations.iter_mut().find(|item| item.name == self.value("location").ok().flatten().unwrap_or_default()) {
                        location.exploration = (location.exploration + 1).min(100);
                    }
                }
                if energy < 20.0 { mood -= 0.03; }
                if energy < 5.0 { mood -= 0.08; }
                health += if energy < 5.0 { -0.02 } else { 0.01 };
                // One low-volume ambient event at most every ten simulated minutes.
                let event_chance = (0.25 + intelligence * 0.001 + creativity * 0.001).min(0.45);
                if minute % 10 == 9 && unit(seed + minute as u64 * 31) < event_chance {
                    let event_roll = unit(seed + minute as u64 * 43);
                    let (delta_energy, delta_mood, delta_health, delta_xp, delta_friendship) = self.insert_simulation_event(cursor, &behavior, event_roll, &mut event_count)?;
                    energy += delta_energy as f32;
                    mood += delta_mood as f32;
                    health += delta_health as f32;
                    friendship += delta_friendship as f32;
                    if let Some(location) = locations.iter_mut().find(|item| item.name == self.value("location").ok().flatten().unwrap_or_default()) {
                        location.exploration = (location.exploration + 1).min(100);
                    }
                    xp += delta_xp;
                }
                let discovery_modifier = (0.55 + curiosity * 0.006 + courage * 0.004).clamp(0.55, 1.6);
                if behavior == "explore" && energy > 30.0 && unit(seed + minute as u64 * 97) < 0.06 * discovery_modifier.min(1.5) {
                    self.generate_npc(cursor, seed + minute as u64)?;
                    mood += 5.0;
                    xp += 10;
                }
                if behavior == "explore" && energy > 30.0 && unit(seed + minute as u64 * 131) < 0.05 * (0.7 + curiosity * 0.006 + courage * 0.004).min(1.5) {
                    self.generate_location(cursor, seed + minute as u64)?;
                    mood += 3.0;
                    curiosity += 1.0;
                }
            }
            energy = energy.clamp(0.0, 100.0);
            mood = mood.clamp(0.0, 100.0);
            health = health.clamp(0.0, 100.0);
            curiosity = curiosity.clamp(0.0, 100.0);
            friendship = friendship.clamp(0.0, 100.0);
        }
        let total = self.value("total_play_time")?.and_then(|v| v.parse::<i64>().ok()).unwrap_or(0) + if offline { 0 } else { elapsed };
        let day_count = (now / 86400) - (self.value("birth_day")?.and_then(|v| v.parse().ok()).unwrap_or(now / 86400)) + 1;
        while xp >= next_xp {
            xp -= next_xp;
            level += 1;
            next_xp = (next_xp as f32 * 1.35).round() as i32;
        }
        let tx = self.db.transaction()?;
        Self::set(&tx, "energy", energy.round() as i32)?;
        Self::set(&tx, "mood", mood.round() as i32)?;
        Self::set(&tx, "health", health.round() as i32)?;
        Self::set(&tx, "curiosity", format!("{:.1}", curiosity))?;
        Self::set(&tx, "friendship", format!("{:.1}", friendship))?;
        Self::set(&tx, "xp", xp)?;
        Self::set(&tx, "level", level)?;
        Self::set(&tx, "next_xp", next_xp)?;
        Self::set(&tx, "current_behavior", &behavior)?;
        Self::set(&tx, "behavior_until", behavior_until)?;
        Self::set(&tx, "total_play_time", total)?;
        Self::set(&tx, "day_count", day_count)?;
        Self::set(&tx, "known_locations", serde_json::to_string(&locations).unwrap())?;
        Self::set(&tx, "last_update", Local::now().to_rfc3339())?;
        Self::set(&tx, "next_normal_check", now + 10 * 60)?;
        tx.commit()
    }

    fn insert_simulation_event(&self, timestamp: i64, behavior: &str, roll: f32, count: &mut i32) -> rusqlite::Result<(i32, i32, i32, i32, i32)> {
        let (event_type, summary, importance, delta_energy, delta_mood, delta_health) = if roll < 0.20 {
            ("social_event", "与熟悉的人交流了一会儿", 0.28, 0, 2, 0)
        } else if roll < 0.28 {
            ("item_event", "整理并获得了一件日常物品", 0.25, 0, 2, 0)
        } else {
            match behavior {
                "play" => ("activity_event", "进行了一会儿轻松的玩耍", 0.15, -1, 3, 0),
                "explore" => ("discovery_event", "观察周围环境，留下了一条新的记录", 0.24, -1, 1, 0),
                "sleep" => ("activity_event", "安静地休息了一会儿", 0.12, 2, 2, 1),
                _ => ("normal_event", "度过了一段平静的时间", 0.10, 0, 1, 0),
            }
        };
        let id = format!("sim-{}-{}", timestamp, *count);
        let now = chrono::DateTime::<chrono::Utc>::from_timestamp(timestamp, 0)
            .map(|value| value.to_rfc3339())
            .unwrap_or_else(|| Local::now().to_rfc3339());
        let delta_xp = 1;
        let delta_friendship = if event_type == "social_event" { 1 } else { 0 };
        let suffix = format!("（能量 {:+}，心情 {:+}，体力 {:+}，探索度 +1，社交 {:+}，经验 +{}）", delta_energy, delta_mood, delta_health, delta_friendship, delta_xp);
        if event_type == "item_event" {
            let mut inventory: Vec<InventoryItem> = self.json("inventory", vec![]);
            inventory.push(InventoryItem { name: "小纪念品".into(), detail: "在日常事件中获得。".into(), icon: "gift".into() });
            self.db.execute("INSERT OR REPLACE INTO world_state(key,value) VALUES ('inventory',?1)", params![serde_json::to_string(&inventory).unwrap()])?;
        }
        self.db.execute("INSERT OR IGNORE INTO events(id,timestamp,type,summary,importance,location,causes,participants) VALUES (?1,?2,?3,?4,?5,?6,'[]','[\"main\"]')",
            params![id, now, event_type, format!("{}{}", summary, suffix), importance, self.value("location")?.unwrap_or_default()])?;
        *count += 1;
        Ok((delta_energy, delta_mood, delta_health, delta_xp, delta_friendship))
    }

    fn generate_npc(&mut self, timestamp: i64, seed: u64) -> rusqlite::Result<()> {
        let names = ["Mika", "Sora", "Nia", "Kaito", "Lena", "Tomo"];
        let roles = ["面包师", "园丁", "修理师", "研究员", "旅行者", "音乐教师"];
        let personalities = ["温和而细心", "好奇健谈", "安静可靠", "热情直接", "谨慎聪明"];
        let index = (seed as usize) % names.len();
        let id = format!("npc-{}", timestamp);
        let name = names[index];
        let role = roles[(seed as usize / 3) % roles.len()];
        let personality = personalities[(seed as usize / 7) % personalities.len()];
        let location = self.value("location")?.unwrap_or_else(|| "Home".into());
        let favorite = ["书签", "热茶", "齿轮", "画笔", "星图"][(seed as usize / 11) % 5];
        let tx = self.db.transaction()?;
        tx.execute("INSERT OR IGNORE INTO npcs(id,name,role,avatar,personality,favorite_item,home_location) VALUES (?1,?2,?3,?4,?5,?6,?7)",
            params![id, name, role, name.chars().next().unwrap_or('N').to_string(), personality, favorite, location])?;
        tx.execute("INSERT OR IGNORE INTO relationships(npc_id,score,stage) VALUES (?1,?2,'acquaintance')",
            params![id, (seed % 21) as i32])?;
        tx.execute("INSERT OR IGNORE INTO events(id,timestamp,type,summary,importance,location,causes,participants) VALUES (?1,?2,'relationship_event',?3,0.72,?4,'[]',?5)",
            params![format!("npc-event-{}", timestamp), Local::now().to_rfc3339(), format!("认识了新的{}：{}，对方是一位{}。", role, name, personality), location, serde_json::to_string(&vec!["main", &id]).unwrap()])?;
        tx.commit()
    }

    fn generate_location(&mut self, timestamp: i64, seed: u64) -> rusqlite::Result<()> {
        let candidates = [
            ("Riverside", "A quiet path beside the water.", "common"),
            ("Clock Tower", "An old tower filled with soft mechanical sounds.", "uncommon"),
            ("Moon Garden", "A hidden garden that only opens at night.", "rare"),
            ("Sky Archive", "A legendary archive above the clouds.", "legendary"),
        ];
        let (name, description, rarity) = candidates[(seed as usize) % candidates.len()];
        let mut locations: Vec<Location> = self.json("known_locations", vec![]);
        if locations.iter().any(|item| item.name == name) { return Ok(()); }
        locations.push(Location { name: name.into(), description: description.into(), exploration: 0, rarity: rarity.into() });
        let tx = self.db.transaction()?;
        Self::set(&tx, "known_locations", serde_json::to_string(&locations).unwrap())?;
        tx.execute("INSERT OR IGNORE INTO events(id,timestamp,type,summary,importance,location,causes,participants) VALUES (?1,?2,'discovery_event',?3,0.62,?4,'[]','[\"main\"]')",
            params![format!("location-event-{}", timestamp), Local::now().to_rfc3339(), format!("发现了新地点：{}", name), name])?;
        tx.commit()
    }

    pub fn reset(&mut self) -> rusqlite::Result<()> {
        self.db.execute_batch("DELETE FROM world_state; DELETE FROM events; DELETE FROM event_progress; DELETE FROM event_threads; DELETE FROM personality_evidence; DELETE FROM relationships; DELETE FROM shared_experiences; DELETE FROM memories; DELETE FROM inventory; DELETE FROM skills; DELETE FROM goals; DELETE FROM personality_traits; DELETE FROM npcs; DELETE FROM characters; DELETE FROM important_people;")?;
        self.seed_defaults()?;
        let locations = r#"[{"name":"家","description":"可以休息和整理物品的地方。","exploration":0,"rarity":"common"},{"name":"学校","description":"学习、完成任务和认识新朋友的地方。","exploration":0,"rarity":"common"},{"name":"便利店","description":"可以买到日常用品和简单食物。","exploration":0,"rarity":"common"},{"name":"公园","description":"适合散步、玩耍和观察环境。","exploration":0,"rarity":"common"},{"name":"街道","description":"连接各个地点的日常道路。","exploration":0,"rarity":"common"}]"#;
        let names = ["小雨", "晴", "阿岚", "小夏", "星野"];
        let seed = Local::now().timestamp_nanos_opt().unwrap_or_default().unsigned_abs() as usize;
        let name = names[seed % names.len()];
        let location = ["家", "学校", "便利店", "公园", "街道"][seed % 5];
        let npc_id = format!("npc-initial-{}", seed);
        let tx = self.db.transaction()?;
        for (key, value) in [("energy", "100"), ("mood", "100"), ("health", "100"), ("intelligence", "10"), ("friendship", "10"), ("curiosity", "10"), ("creativity", "10"), ("courage", "10"), ("location", location), ("current_behavior", "idle"), ("day_count", "1"), ("total_play_time", "0"), ("last_update", &Local::now().to_rfc3339()), ("next_normal_check", &(Local::now().timestamp() + 600).to_string())] {
            Self::set(&tx, key, value)?;
        }
        Self::set(&tx, "known_locations", locations)?;
        Self::set(&tx, "inventory", r#"[{"name":"书包","detail":"一个可以装下日常物品的书包。","icon":"backpack"}]"#)?;
        Self::set(&tx, "skills", "[]")?;
        tx.execute("INSERT INTO npcs(id,name,role,avatar,personality,favorite_item,home_location) VALUES (?1,?2,'朋友',?3,'友善、愿意一起探索','书签',?4)",
            params![npc_id, name, name.chars().next().unwrap_or('友').to_string(), location])?;
        tx.execute("INSERT INTO relationships(npc_id,score,stage) VALUES (?1,10,'friend')", params![npc_id])?;
        tx.execute("INSERT INTO inventory(id,name,quantity,description) VALUES ('backpack','书包',1,'一个可以装下日常物品的书包。')", [])?;
        tx.commit()
    }

    pub fn apply(&mut self, mut p: EventProposal) -> rusqlite::Result<WorldSnapshot> {
        if p.event_type == "no_event" {
            return Err(rusqlite::Error::InvalidParameterName("silent events are disabled".into()));
        }
        if !EVENT_TYPES.contains(&p.event_type.as_str()) { return Err(rusqlite::Error::InvalidParameterName("invalid event type".into())); }
        if p.summary.trim().is_empty() || p.summary.chars().count()>160 { return Err(rusqlite::Error::InvalidParameterName("summary must be 1-160 characters".into())); }
        self.enforce_location_schedule(Local::now())?;
        if self.in_rest_period(Local::now()) {
            p.location = "家".into();
        } else if let Some(location) = self.value("location")? {
            p.location = location;
        }
        if is_sleep_event(&p) && !self.in_rest_period(Local::now()) {
            eprintln!("[event] rejected sleep event outside configured rest period");
            return Err(rusqlite::Error::InvalidParameterName("sleep events are only allowed during the configured rest period".into()));
        }
        let xp=self.number("xp",0); let mut level=self.number("level",1); let mut next=self.number("next_xp",100); let mood=(self.number("mood",50)+p.effects.mood).clamp(0,100); let energy=(self.number("energy",50)+p.effects.energy).clamp(0,100); let health=(self.number("health",100)+p.effects.health).clamp(0,100); let money=self.number("money",0)+p.effects.money; let new_xp_delta=p.xp_delta().clamp(0,50); let mut total=xp+new_xp_delta;
        let mut level_ups = 0;
        while total>=next { total-=next; level+=1; next=(next as f32*1.35).round() as i32; level_ups += 1; }
        let now:DateTime<Local>=Local::now();
        let special_event = matches!(p.event_type.as_str(), "important_event" | "milestone_event" | "level_up");
        let clamp_dimension = |value: f32| {
            let value = (value * 10.0).round() / 10.0;
            if special_event { value.clamp(-1.0, 2.0) } else { value.clamp(-1.0, 1.0) }
        };
        let intelligence=(self.decimal("intelligence",50.0)+clamp_dimension(p.effects.intelligence)).clamp(0.0,100.0);
        let friendship=(self.decimal("friendship",0.0)+clamp_dimension(p.effects.friendship)).clamp(0.0,100.0);
        let curiosity=(self.decimal("curiosity",50.0)+clamp_dimension(p.effects.curiosity)).clamp(0.0,100.0);
        let creativity=(self.decimal("creativity",50.0)+clamp_dimension(p.effects.creativity)).clamp(0.0,100.0);
        let courage=(self.decimal("courage",50.0)+clamp_dimension(p.effects.courage)).clamp(0.0,100.0);
        let reward = if level_ups > 0 { Some(["智力", "好奇心", "社交", "创造力", "勇气"][(now.timestamp_millis().unsigned_abs() as usize) % 5]) } else { None };
        let (intelligence, friendship, curiosity, creativity, courage) = match reward {
            Some("智力") => ((intelligence + 1.0).min(100.0), friendship, curiosity, creativity, courage),
            Some("好奇心") => (intelligence, friendship, (curiosity + 1.0).min(100.0), creativity, courage),
            Some("社交") => (intelligence, (friendship + 1.0).min(100.0), curiosity, creativity, courage),
            Some("创造力") => (intelligence, friendship, curiosity, (creativity + 1.0).min(100.0), courage),
            Some("勇气") => (intelligence, friendship, curiosity, creativity, (courage + 1.0).min(100.0)),
            _ => (intelligence, friendship, curiosity, creativity, courage),
        };
        let reward_suffix = reward.map(|name| format!("（升级：{} +1）", name)).unwrap_or_default();
        let mut normalized_effects = p.effects.clone();
        normalized_effects.intelligence = clamp_dimension(normalized_effects.intelligence);
        normalized_effects.friendship = clamp_dimension(normalized_effects.friendship);
        normalized_effects.curiosity = clamp_dimension(normalized_effects.curiosity);
        normalized_effects.creativity = clamp_dimension(normalized_effects.creativity);
        normalized_effects.courage = clamp_dimension(normalized_effects.courage);
        let id=format!("event-{}",now.timestamp_millis()); let rel=p.relationship().map(|r|(r.target.clone(),r.delta.clamp(-5,5))); let display_summary = format!("{}{}{}", p.summary.trim(), effects_suffix(&normalized_effects), reward_suffix);
        let item_effect=p.effects.item.clone(); let skill_effect=p.effects.skill.clone(); let goal_effect=p.effects.goal.clone();
        let personality_signal=p.effects.personality_signal.clone();
        let mut items: Vec<InventoryItem> = self.json("inventory", vec![]);
        let mut skills: Vec<Skill> = self.json("skills", vec![]);
        let mut goals: Vec<Goal> = self.json("goals", vec![]);
        let old_location = self.value("location")?.unwrap_or_default();
        let tx=self.db.transaction()?;
        let relation = match p.relation.as_deref() {
            Some("continue") | Some("new") | Some("related") | Some("interrupt") | Some("resume") => p.relation.as_deref().unwrap(),
            _ => "new",
        };
        let duration = p.estimated_duration.unwrap_or(if p.event_type == "activity_event" { 20 } else { 0 }).clamp(0, 40);
        let active_thread_id = tx.query_row("SELECT id FROM event_threads WHERE status IN ('planned','active','paused') AND location=?1 ORDER BY last_update_time DESC LIMIT 1", params![&p.location], |row| row.get::<_, String>(0)).optional().ok().flatten();
        let requested_thread_exists = p.thread_id.as_deref().and_then(|value| {
            tx.query_row("SELECT id FROM event_threads WHERE id=?1", params![value], |row| row.get::<_, String>(0)).optional().ok().flatten()
        });
        let thread_id = requested_thread_exists.or_else(|| {
            tx.query_row("SELECT id FROM event_threads WHERE status IN ('planned','active','paused') AND location=?1 ORDER BY last_update_time DESC LIMIT 1", params![&p.location], |row| row.get::<_, String>(0)).optional().ok().flatten()
        });
        let inferred_continuation = matches!(p.relation.as_deref(), None | Some("new"))
            && active_thread_id.is_some()
            && (p.estimated_duration.is_none() || duration < 10)
            && matches!(p.event_type.as_str(), "activity_event" | "discovery_event" | "skill_event" | "relationship_event");
        let effective_relation = if inferred_continuation { "continue" } else { relation };
        let effective_thread_id = if inferred_continuation { active_thread_id.clone() } else { thread_id };
        let is_thread = duration >= 10 || effective_thread_id.is_some() || matches!(effective_relation, "continue" | "related" | "interrupt" | "resume");
        if is_thread {
            let target_thread = if effective_relation == "new" || effective_relation == "related" || effective_thread_id.is_none() {
                let new_id = p.thread_id.clone().unwrap_or_else(|| format!("thread-{}", now.timestamp_millis()));
                let state = p.progress.as_ref().and_then(|value| value.state.as_deref()).unwrap_or("active");
                let initial_progress = p.progress.as_ref().map(|value| value.progress.clamp(0.0, 1.0)).unwrap_or(0.0);
                tx.execute("INSERT OR IGNORE INTO event_threads(id,title,summary,type,start_time,last_update_time,end_time,estimated_duration,actual_duration,status,progress,importance,location,participants) VALUES (?1,?2,?3,?4,?5,?5,NULL,?6,NULL,?7,?8,?9,?10,?11)", params![
                    &new_id, p.title.as_deref().unwrap_or(p.summary.trim()), &display_summary, &p.event_type,
                    now.to_rfc3339(), duration.max(10), state, initial_progress, p.importance.clamp(0.0, 1.0),
                    &p.location, serde_json::to_string(&p.participants).unwrap()
                ])?;
                Some(new_id)
            } else {
                effective_thread_id
            };
            if let Some(thread_id) = target_thread {
                let previous: Option<String> = tx.query_row("SELECT last_update_time FROM event_threads WHERE id=?1", params![&thread_id], |row| row.get(0)).optional()?;
                let elapsed = previous.as_deref().and_then(|value| DateTime::parse_from_rfc3339(value).ok()).map(|value| (now.with_timezone(value.offset()) - value).num_seconds()).unwrap_or(600);
                let current_progress: f32 = tx.query_row("SELECT progress FROM event_threads WHERE id=?1", params![&thread_id], |row| row.get(0)).optional()?.unwrap_or(0.0);
                let progress_count: i32 = tx.query_row("SELECT COUNT(*) FROM event_progress WHERE thread_id=?1", params![&thread_id], |row| row.get(0))?;
                let fallback_progress = ProgressUpdate {
                    summary: p.summary.trim().to_string(),
                    progress: (current_progress + 0.2).min(0.95),
                    state: Some("active".into()),
                };
                let update = if progress_count < 4 && (p.progress.is_some() || elapsed >= 5 * 60) {
                    Some(p.progress.as_ref().unwrap_or(&fallback_progress))
                } else { None };
                let status = p.progress.as_ref().and_then(|value| value.state.as_deref()).unwrap_or(if effective_relation == "interrupt" { "interrupted" } else if effective_relation == "resume" { "active" } else { "active" });
                let finishing_update = progress_count >= 3 && update.is_some();
                let status = if finishing_update { "completed" } else { status };
                let progress = if finishing_update { 1.0 } else { p.progress.as_ref().map(|value| value.progress.clamp(0.0, 1.0)).unwrap_or(if status == "completed" { 1.0 } else { 0.0 }) };
                if let Some(update) = update {
                    let progress_id = format!("progress-{}", now.timestamp_millis());
                    tx.execute("INSERT INTO event_progress(id,thread_id,timestamp,summary,progress,state,effects) VALUES (?1,?2,?3,?4,?5,?6,?7)", params![
                        progress_id, &thread_id, now.to_rfc3339(), update.summary.trim(),
                        update.progress.clamp(0.0, 1.0), status, serde_json::to_string(&normalized_effects).unwrap()
                    ])?;
                }
                let terminal = matches!(status, "completed" | "failed" | "abandoned" | "interrupted");
                let actual_duration = if terminal {
                    let start = previous.as_deref().and_then(|value| DateTime::parse_from_rfc3339(value).ok()).map(|value| value.timestamp());
                    Some(((now.timestamp() - start.unwrap_or(now.timestamp())).max(0) / 60) as i32)
                } else { None };
                tx.execute("UPDATE event_threads SET last_update_time=?1,status=?2,progress=?3,end_time=?4,actual_duration=?5 WHERE id=?6", params![
                    now.to_rfc3339(), status, progress, if terminal { Some(now.to_rfc3339()) } else { None::<String> }, actual_duration, &thread_id
                ])?;
            }
        } else {
            tx.execute("INSERT INTO events(id,timestamp,type,summary,importance,location,causes,participants) VALUES (?1,?2,?3,?4,?5,?6,?7,?8)",params![&id,now.to_rfc3339(),&p.event_type,&display_summary,p.importance.clamp(0.0,1.0),&p.location,serde_json::to_string(&p.causes).unwrap(),serde_json::to_string(&p.participants).unwrap()])?;
        }
        Self::set(&tx,"xp",total)?; Self::set(&tx,"level",level)?; Self::set(&tx,"next_xp",next)?; Self::set(&tx,"mood",mood)?; Self::set(&tx,"energy",energy)?; Self::set(&tx,"health",health)?; Self::set(&tx,"intelligence",format!("{:.1}",intelligence))?; Self::set(&tx,"friendship",format!("{:.1}",friendship))?; Self::set(&tx,"curiosity",format!("{:.1}",curiosity))?; Self::set(&tx,"creativity",format!("{:.1}",creativity))?; Self::set(&tx,"courage",format!("{:.1}",courage))?; Self::set(&tx,"money",money)?; Self::set(&tx,"location",&p.location)?; if p.location != old_location { Self::set(&tx,"last_location_change",now.timestamp())?; } Self::set(&tx,"last_update",now.to_rfc3339())?; Self::set(&tx,"next_normal_check",now.timestamp()+1800)?;
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
        if p.memory { tx.execute("INSERT INTO memories(id,event_id,summary,created_at) VALUES (?1,?2,?3,?4)",params![&id,&id,&display_summary,now.to_rfc3339()])?; }
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

fn unit(seed: u64) -> f32 {
    let mut value = seed ^ 0x9E3779B97F4A7C15;
    value = value.wrapping_mul(0xBF58476D1CE4E5B9);
    value ^= value >> 27;
    (value as f64 / u64::MAX as f64) as f32
}

fn is_sleep_event(proposal: &EventProposal) -> bool {
    if proposal.event_type == "sleep_event" {
        return true;
    }
    if proposal.event_type != "activity_event" {
        return false;
    }
    let text = format!("{} {}", proposal.summary, proposal.title.as_deref().unwrap_or_default());
    ["\u{7761}", "\u{5165}\u{7761}", "\u{7761}\u{89c9}", "\u{7761}\u{53bb}", "\u{6c89}\u{6c89}"]
        .iter()
        .any(|keyword| text.contains(keyword))
}

fn effects_suffix(effects: &EventEffects) -> String {
    let mut changes = Vec::new();
    if effects.energy != 0 { changes.push(format!("能量 {:+}", effects.energy)); }
    if effects.mood != 0 { changes.push(format!("心情 {:+}", effects.mood)); }
    if effects.health != 0 { changes.push(format!("体力 {:+}", effects.health)); }
    if effects.intelligence != 0.0 { changes.push(format!("智力 {:+.1}", effects.intelligence)); }
    if effects.curiosity != 0.0 { changes.push(format!("好奇心 {:+.1}", effects.curiosity)); }
    if effects.friendship != 0.0 { changes.push(format!("社交 {:+.1}", effects.friendship)); }
    if effects.creativity != 0.0 { changes.push(format!("创造力 {:+.1}", effects.creativity)); }
    if effects.courage != 0.0 { changes.push(format!("勇气 {:+.1}", effects.courage)); }
    if effects.money != 0 { changes.push(format!("金币 {:+}", effects.money)); }
    if effects.exploration != 0 { changes.push(format!("探索度 {:+}", effects.exploration)); }
    if effects.xp != 0 { changes.push(format!("经验 {:+}", effects.xp)); }
    if effects.item.is_some() { changes.push("获得物品 +1".into()); }
    if changes.is_empty() { String::new() } else { format!("（{}）", changes.join("，")) }
}

fn choose_behavior(roll: f32, energy: f32, mood: f32, intelligence: f32, friendship: f32, curiosity: f32, creativity: f32, courage: f32) -> String {
    let mut weights = vec![
        ("idle", 25.0), ("observe", 15.0), ("play", 15.0), ("sleep", 10.0),
        ("explore", 10.0), ("work", 10.0), ("social", 8.0), ("special", 7.0),
    ];
    weights[2].1 *= 0.8 + creativity / 100.0;
    weights[4].1 *= 0.7 + (curiosity + courage) / 100.0;
    weights[5].1 *= 0.7 + intelligence / 100.0;
    weights[6].1 *= 0.7 + friendship / 100.0;
    weights[7].1 *= 0.7 + (creativity + courage) / 100.0;
    if energy < 20.0 {
        for (name, weight) in &mut weights {
            if *name == "play" || *name == "explore" || *name == "work" || *name == "special" { *weight *= 0.3; }
            if *name == "sleep" { *weight *= 3.0; }
        }
    }
    if mood > 80.0 {
        for (name, weight) in &mut weights {
            if *name == "explore" || *name == "social" { *weight *= 1.5; }
        }
    }
    let total: f32 = weights.iter().map(|(_, weight)| *weight).sum();
    let mut cursor = roll.clamp(0.0, 0.9999) * total;
    for (name, weight) in weights {
        if cursor < weight { return name.into(); }
        cursor -= weight;
    }
    "idle".into()
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
    use chrono::TimeZone;

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
            participants: vec!["main".into()], causes: vec!["event-old".into()], memory: true, ..Default::default()
        }).unwrap();
        assert_eq!(snapshot.mood, 100);
        assert_eq!(snapshot.energy, 0);
        assert_eq!(snapshot.events[0].participants, vec!["main"]);
        assert_eq!(snapshot.events[0].causes, vec!["event-old"]);
    }

    #[test]
    fn normal_dimension_effects_are_limited_to_one_decimal_point() {
        let mut engine = engine();
        let snapshot = engine.apply(EventProposal {
            event_type: "normal_event".into(), summary: "属性测试".into(), importance: 0.4,
            location: "房间".into(),
            effects: EventEffects { creativity: 2.0, friendship: -1.26, ..Default::default() },
            participants: vec!["main".into()], causes: vec![], memory: false, ..Default::default()
        }).unwrap();
        assert_eq!(snapshot.creativity, 51.0);
        assert_eq!(snapshot.friendship, 0.0);
        assert!(snapshot.events[0].summary.contains("创造力 +1.0"));
        assert!(!snapshot.events[0].summary.contains("创造力 +2"));
    }

    #[test]
    fn activity_without_duration_creates_expandable_thread() {
        let mut engine = engine();
        let snapshot = engine.apply(EventProposal {
            event_type: "activity_event".into(), summary: "去图书馆学习".into(), importance: 0.4,
            location: "图书馆".into(), effects: EventEffects::default(),
            participants: vec!["main".into()], causes: vec![], memory: false,
            relation: None, ..Default::default()
        }).unwrap();
        assert_eq!(snapshot.events.len(), 0);
        assert_eq!(snapshot.event_threads.len(), 1);
        assert_eq!(snapshot.event_threads[0].estimated_duration, 20);
        assert_eq!(snapshot.event_threads[0].updates.len(), 0);
    }

    #[test]
    fn thread_plans_at_most_four_progress_updates() {
        let mut engine = engine();
        let first = engine.apply(EventProposal {
            event_type: "activity_event".into(), summary: "开始学习".into(), importance: 0.4,
            location: "图书馆".into(), estimated_duration: Some(30),
            participants: vec!["main".into()], ..Default::default()
        }).unwrap();
        let thread_id = first.event_threads[0].id.clone();
        for index in 0..3 {
            engine.db.execute(
                "INSERT INTO event_progress(id,thread_id,timestamp,summary,progress,state) VALUES (?1,?2,datetime('now',?3),?4,?5,'active')",
                params![format!("seed-progress-{index}"), &thread_id, format!("-{} minutes", 10 + index), format!("进展 {index}"), (index as f32 + 1.0) / 5.0],
            ).unwrap();
        }
        let old_time = (Local::now() - chrono::Duration::minutes(10)).to_rfc3339();
        engine.db.execute("UPDATE event_threads SET last_update_time=?1 WHERE id=?2", params![old_time, &thread_id]).unwrap();
        let snapshot = engine.apply(EventProposal {
            event_type: "activity_event".into(), summary: "继续学习".into(), importance: 0.4,
            location: "图书馆".into(), relation: Some("continue".into()), thread_id: Some(thread_id),
            progress: Some(ProgressUpdate { summary: "继续完成练习".into(), progress: 0.9, state: Some("active".into()) }),
            participants: vec!["main".into()], ..Default::default()
        }).unwrap();
        assert_eq!(snapshot.event_threads[0].updates.len(), 4);
        assert_eq!(snapshot.event_threads[0].status, "completed");
    }

    #[test]
    fn explicit_progress_is_saved_without_waiting_five_minutes() {
        let mut engine = engine();
        let first = engine.apply(EventProposal {
            event_type: "activity_event".into(), summary: "继续阅读".into(), importance: 0.2,
            location: "家".into(), estimated_duration: Some(20),
            participants: vec!["main".into()], ..Default::default()
        }).unwrap();
        let thread_id = first.event_threads[0].id.clone();
        let snapshot = engine.apply(EventProposal {
            event_type: "normal_event".into(), summary: "你合上书去洗漱".into(), importance: 0.2,
            location: "家".into(), relation: Some("continue".into()), thread_id: Some(thread_id),
            progress: Some(ProgressUpdate { summary: "你合上书去洗漱".into(), progress: 0.5, state: Some("active".into()) }),
            participants: vec!["main".into()], ..Default::default()
        }).unwrap();
        assert_eq!(snapshot.events.len(), 0);
        assert_eq!(snapshot.event_threads[0].updates.len(), 1);
        assert_eq!(snapshot.event_threads[0].updates[0].summary, "你合上书去洗漱");
    }

    #[test]
    fn rest_period_forces_location_home() {
        let mut engine = engine();
        let tx = engine.db.transaction().unwrap();
        Engine::set(&tx, "location", "学校").unwrap();
        Engine::set(&tx, "rest_start", 22).unwrap();
        Engine::set(&tx, "rest_end", 8).unwrap();
        tx.commit().unwrap();
        engine.enforce_location_schedule(Local.with_ymd_and_hms(2026, 9, 4, 23, 0, 0).unwrap()).unwrap();
        assert_eq!(engine.value("location").unwrap().unwrap(), "家");
    }

    #[test]
    fn non_rest_period_forces_location_change_after_four_hours() {
        let mut engine = engine();
        let now = Local.with_ymd_and_hms(2026, 9, 4, 12, 0, 0).unwrap();
        let tx = engine.db.transaction().unwrap();
        Engine::set(&tx, "location", "家").unwrap();
        Engine::set(&tx, "known_locations", r#"[{"name":"家","description":"","exploration":0,"rarity":"common"},{"name":"学校","description":"","exploration":0,"rarity":"common"}]"#).unwrap();
        Engine::set(&tx, "last_location_change", now.timestamp() - 4 * 3600).unwrap();
        Engine::set(&tx, "rest_start", 22).unwrap();
        Engine::set(&tx, "rest_end", 8).unwrap();
        tx.commit().unwrap();
        engine.enforce_location_schedule(now).unwrap();
        assert_eq!(engine.value("location").unwrap().unwrap(), "学校");
    }

    #[test]
    fn relationship_updates_stage() {
        let mut engine = engine();
        engine.db.execute("INSERT INTO npcs(id,name,role,avatar) VALUES ('yuki','Yuki','traveler','Y')", []).unwrap();
        engine.db.execute("INSERT INTO relationships(npc_id,score,stage) VALUES ('yuki',18,'acquaintance')", []).unwrap();
        let snapshot = engine.apply(EventProposal {
            event_type: "relationship_event".into(), summary: "关系变化".into(), importance: 0.5,
            location: "图书馆".into(), effects: EventEffects { relationship: Some(RelationshipEffect { target: "yuki".into(), delta: 5 }), ..Default::default() },
            participants: vec!["main".into(), "yuki".into()], causes: vec![], memory: false, ..Default::default()
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
