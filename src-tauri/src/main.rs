#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use chrono::{Local, Timelike};
mod world;
mod llm;
mod scheduler;

use std::sync::Mutex;
use std::time::Duration;
use tauri::{Emitter, Manager, Position, PhysicalPosition, State, WindowEvent};
use serde::Deserialize;
use serde_json::{Map, Value};
use world::{Engine, EventProposal, InitialWorldConfig, InventoryItem, Location, NpcEffect, WorldSnapshot};
use llm::ProviderConfig;

#[cfg(windows)]
fn configure_pet_window(window: &tauri::WebviewWindow) {
    #[repr(C)]
    struct Margins { cx_left_width: i32, cx_right_width: i32, cy_top_height: i32, cy_bottom_height: i32 }
    use windows_sys::Win32::Graphics::Dwm::{DwmExtendFrameIntoClientArea, DwmSetWindowAttribute, DWMWA_NCRENDERING_POLICY, DWMNCRP_DISABLED};
    use windows_sys::Win32::UI::WindowsAndMessaging::{GetWindowLongPtrW, SetWindowLongPtrW, SetWindowPos, GWL_EXSTYLE, GWL_STYLE, SWP_FRAMECHANGED, SWP_NOMOVE, SWP_NOSIZE, SWP_NOACTIVATE, SWP_NOZORDER, HWND_TOP, WS_BORDER, WS_CAPTION, WS_DLGFRAME, WS_EX_APPWINDOW, WS_EX_CLIENTEDGE, WS_EX_DLGMODALFRAME, WS_EX_LAYERED, WS_EX_NOACTIVATE, WS_EX_STATICEDGE, WS_EX_TOOLWINDOW, WS_EX_WINDOWEDGE, WS_POPUP, WS_THICKFRAME};
    let Ok(hwnd) = window.hwnd() else {
        eprintln!("[window] failed to get pet HWND");
        return;
    };
    unsafe {
        let style = GetWindowLongPtrW(hwnd.0 as _, GWL_EXSTYLE);
        // Remove every native edge/taskbar style that can reappear after activation.
        let edge_flags = (WS_EX_APPWINDOW | WS_EX_CLIENTEDGE | WS_EX_DLGMODALFRAME | WS_EX_STATICEDGE | WS_EX_WINDOWEDGE) as isize;
        let flags = (style & !edge_flags) | WS_EX_LAYERED as isize | WS_EX_NOACTIVATE as isize | WS_EX_TOOLWINDOW as isize;
        SetWindowLongPtrW(hwnd.0 as _, GWL_EXSTYLE, flags);
        let window_style = GetWindowLongPtrW(hwnd.0 as _, GWL_STYLE);
        let frame_flags = (WS_BORDER | WS_CAPTION | WS_DLGFRAME | WS_THICKFRAME) as isize;
        SetWindowLongPtrW(hwnd.0 as _, GWL_STYLE, (window_style & !frame_flags) | WS_POPUP as isize);
        SetWindowPos(hwnd.0 as _, HWND_TOP, 0, 0, 0, 0, SWP_FRAMECHANGED | SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER | SWP_NOACTIVATE);
        let policy = DWMNCRP_DISABLED;
        let result = DwmSetWindowAttribute(hwnd.0 as _, DWMWA_NCRENDERING_POLICY as _, &policy as *const _ as _, std::mem::size_of_val(&policy) as u32);
        // Extend the client area through the entire native frame. This removes the
        // single-pixel top edge that WebView2 can leave on transparent popup windows.
        let margins = Margins { cx_left_width: -1, cx_right_width: -1, cy_top_height: -1, cy_bottom_height: -1 };
        let glass_result = DwmExtendFrameIntoClientArea(hwnd.0 as _, &margins as *const Margins as *const _);
        eprintln!("[window] pet native styles applied hwnd={} dwm_result={} glass_result={}", hwnd.0 as usize, result, glass_result);
    }
}

#[cfg(not(windows))]
fn configure_pet_window(_window: &tauri::WebviewWindow) {}

#[tauri::command]
fn get_world(engine: State<'_, Mutex<Engine>>) -> Result<WorldSnapshot, String> {
    let mut guard = engine.lock().map_err(|e| e.to_string())?;
    let _ = guard.scheduler_tick();
    guard.snapshot().map_err(|e| e.to_string())
}

#[tauri::command]
fn apply_proposal(app: tauri::AppHandle, engine: State<'_, Mutex<Engine>>, proposal: EventProposal) -> Result<WorldSnapshot, String> {
    eprintln!("[event] applying manual proposal type={} summary={}", proposal.event_type, proposal.summary);
    let snapshot = engine.lock().map_err(|e| e.to_string())?.apply(proposal).map_err(|e| e.to_string())?;
    let _ = app.emit("world-updated", &snapshot);
    Ok(snapshot)
}

#[tauri::command]
fn reset_world(engine: State<'_, Mutex<Engine>>) -> Result<(), String> {
    eprintln!("[world] reset requested");
    engine.lock().map_err(|e| e.to_string())?.reset().map_err(|e| e.to_string())
}

#[tauri::command]
fn initialize_world(app: tauri::AppHandle, engine: State<'_, Mutex<Engine>>, config: InitialWorldConfig) -> Result<WorldSnapshot, String> {
    eprintln!("[world] initialize requested name={}", config.name);
    let snapshot = engine.lock().map_err(|e| e.to_string())?.initialize(config).map_err(|e| e.to_string())?;
    let _ = app.emit("world-updated", &snapshot);
    Ok(snapshot)
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct InitialAiPayload {
    item: InitialAiItem,
    locations: Vec<Location>,
    npc: NpcEffect,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct InitialAiItem {
    name: String,
    description: String,
}

#[tauri::command]
async fn initialize_world_ai(app: tauri::AppHandle, engine: State<'_, Mutex<Engine>>, base_url: String, model: String, api_key: Option<String>, language: Option<String>, mut config: InitialWorldConfig) -> Result<WorldSnapshot, String> {
    if base_url.trim().is_empty() || model.trim().is_empty() || api_key.as_deref().unwrap_or_default().trim().is_empty() {
        return Err("Base URL, model and API key are required for AI initialization".into());
    }
    let prompt = format!(
        "Create the initial world seed for a desktop pet life simulation. Return exactly one valid JSON object with no Markdown. Schema: {{\"item\":{{\"name\":\"string\",\"description\":\"string\"}},\"locations\":[{{\"name\":\"string\",\"description\":\"string\",\"exploration\":0,\"rarity\":\"common\"}}],\"npc\":{{\"id\":\"zhaoran\",\"name\":\"zhaoran\",\"role\":\"string\",\"personality\":\"string\",\"favoriteItem\":\"string\",\"homeLocation\":\"string\",\"relationship\":0-100,\"relationshipNote\":\"string\",\"avatar\":\"\"}}}}. Generate exactly 4 basic locations based on the world setting, not 5. Generate exactly 1 starting item with a concrete useful description. Generate exactly 1 starting person: name and id must be zhaoran, but role/personality/favoriteItem/homeLocation/relationship/relationshipNote must be generated from the setting. role must be an identity or occupation such as 学生, 老师, 店员, 医生, 图书管理员, 插画师, 程序员, 研究员; never use 朋友/friend as role. If zhaoran's home is 家/Home, write homeLocation as zhaoran的家. Keep all Chinese text natural and concise. WORLD SETTING: {}. MAIN CHARACTER: name={}, tags={}, personality={}, experiences={}, interests={}, behavior={}. Sprite grid: {} columns x {} rows.",
        config.world_background,
        config.name,
        config.character_tags,
        config.character_description,
        config.character_experiences,
        config.character_interests,
        config.character_behavior,
        config.sprite_columns,
        config.sprite_rows
    );
    eprintln!("[world] AI initialization request model={} prompt_chars={}", model, prompt.chars().count());
    let raw = llm::generate(ProviderConfig {
        base_url,
        model,
        api_key: api_key.unwrap_or_default(),
        language: language.unwrap_or_else(|| "zh".into()),
    }, prompt).await?;
    let value = llm::parse_proposal(&raw)?;
    let generated: InitialAiPayload = serde_json::from_value(value).map_err(|error| {
        let message = format!("AI initialization schema mismatch: {}", error);
        eprintln!("[world] {}", message);
        message
    })?;
    config.inventory = vec![InventoryItem { name: generated.item.name.clone(), detail: generated.item.description, icon: "backpack".into() }];
    config.items = vec![generated.item.name];
    config.locations = generated.locations.into_iter().take(4).map(|mut location| {
        location.exploration = 0;
        if location.rarity.trim().is_empty() { location.rarity = "common".into(); }
        location
    }).collect();
    config.location = config.locations.first().map(|location| location.name.clone()).unwrap_or_else(|| config.location.clone());
    config.npc = Some(generated.npc);
    let mut guard = engine.lock().map_err(|e| e.to_string())?;
    let _ = guard.initialize(config.clone()).map_err(|e| e.to_string())?;
    let snapshot = guard.apply_initial_assets(&config).map_err(|e| e.to_string())?;
    let _ = app.emit("world-updated", &snapshot);
    Ok(snapshot)
}

#[tauri::command]
fn scheduler_tick(engine: State<'_, Mutex<Engine>>) -> Result<Option<String>, String> {
    engine.lock().map_err(|e| e.to_string())?.scheduler_tick().map_err(|e| e.to_string())
}

#[tauri::command]
fn set_rest_hours(app: tauri::AppHandle, engine: State<'_, Mutex<Engine>>, start: i32, end: i32) -> Result<WorldSnapshot, String> {
    let mut guard = engine.lock().map_err(|e| e.to_string())?;
    guard.set_rest_hours(start, end).map_err(|e| e.to_string())?;
    let _ = guard.scheduler_tick();
    let snapshot = guard.snapshot().map_err(|e| e.to_string())?;
    let _ = app.emit("world-updated", &snapshot);
    Ok(snapshot)
}

#[tauri::command]
fn toggle_chronicle(app: tauri::AppHandle) -> Result<bool, String> {
    let chronicle = app.get_webview_window("chronicle").ok_or_else(|| "chronicle window not found".to_string())?;
    let visible = chronicle.is_visible().map_err(|e| e.to_string())?;
    eprintln!("[window] toggle chronicle current_visible={}", visible);
    if visible {
        chronicle.hide().map_err(|e| e.to_string())?;
        Ok(false)
    } else {
        if let Some(pet) = app.get_webview_window("pet") {
            if let Ok(position) = pet.outer_position() {
                let _ = chronicle.set_position(Position::Physical(PhysicalPosition::new(position.x + 245, position.y)));
            }
        }
        chronicle.show().map_err(|e| e.to_string())?;
        chronicle.set_focus().map_err(|e| e.to_string())?;
        Ok(true)
    }
}

#[tauri::command]
async fn test_provider(base_url: String, model: String, api_key: Option<String>) -> Result<String, String> {
    eprintln!("[provider-test] base_url={} model={} api_key_present={}", base_url, model, api_key.as_deref().is_some_and(|key| !key.trim().is_empty()));
    let result = llm::test_connection(ProviderConfig { base_url, model, api_key: api_key.unwrap_or_default(), language: "zh".into() }).await;
    eprintln!("[provider-test] result={}", result.as_ref().map(|value| value.as_str()).unwrap_or_else(|error| error.as_str()));
    result
}

#[tauri::command]
async fn generate_event(app: tauri::AppHandle, engine: State<'_, Mutex<Engine>>, base_url: String, model: String, api_key: Option<String>, language: Option<String>, character_context: Option<String>, rest_start: Option<i32>, rest_end: Option<i32>) -> Result<WorldSnapshot, String> {
    let start = rest_start.unwrap_or(22).clamp(0, 23);
    let end = rest_end.unwrap_or(8).clamp(0, 23);
    let (snapshot, memory, location_due) = {
        let mut guard = engine.lock().map_err(|e| e.to_string())?;
        guard.set_rest_hours(start, end).map_err(|e| e.to_string())?;
        guard.scheduler_tick().map_err(|e| e.to_string())?;
        let snapshot = guard.snapshot().map_err(|e| e.to_string())?;
        let memory = guard.memory_context();
        let location_due = guard.location_change_due();
        (snapshot, memory, location_due)
    };
    let now = Local::now();
    let hour = now.hour() as i32;
    let in_rest_period = if start == end { true } else if start < end { hour >= start && hour < end } else { hour >= start || hour < end };
    if in_rest_period {
        let sleep_id = format!("sleep-{}", now.format("%Y-%m-%d"));
        let relation = if snapshot.event_threads.iter().any(|thread| thread.id == sleep_id) { Some("continue".into()) } else { Some("new".into()) };
        let proposal = EventProposal {
            event_type: "activity_event".into(),
            summary: "你已沉沉睡去……".into(),
            importance: 0.1,
            location: snapshot.location.clone(),
            effects: Default::default(),
            participants: vec!["main".into()],
            causes: vec!["休息时间".into()],
            memory: false,
            relation,
            thread_id: Some(sleep_id),
            title: None,
            estimated_duration: Some(10),
            progress: None,
        };
        eprintln!("[event] rest period active {}:00-{}:00; using fixed sleep event", start, end);
        let updated = engine.lock().map_err(|e| e.to_string())?.apply(proposal).map_err(|e| e.to_string())?;
        let _ = app.emit("world-updated", &updated);
        return Ok(updated);
    }
    let event_threads_for_prompt: Vec<_> = snapshot.event_threads.iter()
        .filter(|thread| in_rest_period || !thread.id.starts_with("sleep-"))
        .filter(|thread| matches!(thread.status.as_str(), "planned" | "active" | "paused"))
        .collect();
    let latest_thread = snapshot.event_threads.iter()
        .filter(|thread| in_rest_period || !thread.id.starts_with("sleep-"))
        .max_by(|left, right| left.last_update_time.cmp(&right.last_update_time));
    let latest_instant = snapshot.events.first();
    let previous_event = match (latest_thread, latest_instant) {
        (Some(thread), Some(event)) if thread.last_update_time >= event.timestamp => serde_json::json!({
            "kind": "thread",
            "id": thread.id,
            "title": thread.title,
            "summary": thread.summary,
            "status": thread.status,
            "progress": thread.progress,
            "location": thread.location,
            "updates": thread.updates,
        }),
        (Some(thread), _) => serde_json::json!({
            "kind": "thread",
            "id": thread.id,
            "title": thread.title,
            "summary": thread.summary,
            "status": thread.status,
            "progress": thread.progress,
            "location": thread.location,
            "updates": thread.updates,
        }),
        (_, Some(event)) => serde_json::json!({
            "kind": "instant",
            "id": event.id,
            "type": event.event_type,
            "summary": event.summary,
            "location": event.location,
        }),
        _ => serde_json::json!(null),
    };
    let prompt = format!(
        "Generate one concrete causal world event, then compare it with PREVIOUS TOP-LEVEL EVENT before deciding its relation. Return exactly one valid JSON object, with no Markdown or explanation. Required fields: event_type, summary, importance 0..1, location, effects, participants, causes, memory, relation (exactly continue/new/related/interrupt/resume), thread_id (string or null), title (string or null), estimated_duration (integer or null), progress (object or null with summary, progress 0..1, state planned/active/paused/completed/interrupted/failed/abandoned). The summary must mainly begin with “你……”, speak directly to the player like a character casually chatting about what happened today. Use simple, concrete, everyday Simplified Chinese, like a short chat message. Do not write literary, poetic, atmospheric or novel-like prose; avoid piling up adjectives, metaphors and abstract descriptions. Prefer concrete actions, ordinary objects, short reactions and natural complaints. Ordinary small events should usually be 1-2 sentences; events with meaningful content may naturally use 3-5 sentences. Do not repeat facts to fill space. Add light teasing, sarcasm, self-deprecation or small jokes only when it fits the character and situation. If the current event is the continuation, advancement, next step, or intermediate state of the previous event, MUST return relation continue and the previous thread id; it must become a child Progress, never a new top-level event. This applies even when the wording changes, as long as the same task/activity/story is still underway. Use new only when it is a separate instantaneous event or a genuinely new activity. Use related for a separate activity connected to the previous one, interrupt when the previous activity is stopped by this event, and resume when the interrupted activity starts again. A continuous activity must use estimated_duration 10..40. Progress updates must be meaningful and at least 5 minutes apart. For a manual event action, do not return no_event. Do not modify state directly. STATE: {}. PREVIOUS TOP-LEVEL EVENT: {}. ACTIVE EVENT THREADS: {}. MEMORY: {}. CHARACTER: {}. Current local time: {}. A location change opportunity is {}. If the opportunity is false, keep the current location exactly. If true, decide whether to move based on time, behavior, energy, mood, weather and known locations; if moving, choose a different known location and describe the travel.",
        serde_json::to_string(&snapshot).map_err(|e| e.to_string())?,
        serde_json::to_string(&previous_event).map_err(|e| e.to_string())?,
        serde_json::to_string(&event_threads_for_prompt).map_err(|e| e.to_string())?,
        memory,
        character_context.unwrap_or_default(),
        snapshot.world_time,
        if location_due { "available" } else { "not available" }
    );
    let prompt = format!(
        "{}\nAdditional hard rules: plan each event thread to have 0 to 4 child progress updates based on its complexity. A simple event may have no progress updates. A complex activity may use 1, 2, 3 or at most 4 meaningful updates. Never create more than 4. The prompt includes the current update count: if there are already 3 updates, make the next update the final meaningful step and mark the thread completed; if there are 4, do not request another update and end the thread. If relation is continue/resume/interrupt and progress is not null, progress.summary must be the same complete conversational sentence quality as summary, not a shortened label. If the event introduces a new person/NPC, effects.npc is required and must be an object with camelCase fields: id, name, role, personality, favoriteItem, homeLocation, relationship, relationshipNote, avatar. role means occupation/job or social identity, such as 学生, 老师, 店员, 医生, 图书管理员, 插画师, 程序员, 研究员, 社团成员. Never use 朋友/friend as role; friendship belongs in relationshipNote and relationship score. personality means character traits. favoriteItem must be a concrete like/preference, not empty. relationship is the current 0..100 relationship score with the main character. relationshipNote explains how this person relates to the main character. If homeLocation is 家/Home, write it as NAME的家, for example zhaoran的家. Set avatar to an empty string; the app chooses a random transparent PNG avatar. If no new person is created, omit effects.npc or set it to null. For each five-dimensional attribute, sample a probability score from a normal distribution truncated to [0,1], centered at 0.5. A score near 0.5 means no change or a very small change; scores above 0.5 indicate a positive change and scores below 0.5 indicate a negative change. Use the exact local time, current behavior, energy, mood and event outcome to shift the distribution tendency: rest/success/enjoyable activities shift it upward, fatigue/failure/late-night strain shift it downward. The absolute delta must be derived from the distance from 0.5, rounded to one decimal, and remain within the normal-event limit of -1.0..1.0. Do not force every event to change an attribute.",
        prompt
    );
    let config = ProviderConfig {
        base_url,
        model,
        api_key: api_key.unwrap_or_default(),
        language: language.unwrap_or_else(|| "zh".into()),
    };
    eprintln!("[event] generating event with configured provider");
    let raw = llm::generate(config, prompt).await?;
    let mut value = llm::parse_proposal(&raw)?;
    normalize_proposal_value(&mut value);
    eprintln!("[event] parsed proposal json={}", value);
    let mut proposal = serde_json::from_value::<EventProposal>(value).map_err(|error| {
        let message = format!("LLM event schema mismatch: {}", error);
        eprintln!("[event] {}", message);
        message
    })?;
    if !location_due || proposal.location.trim().is_empty() {
        proposal.location = snapshot.location.clone();
    }
    if proposal.event_type == "no_event" {
        let message = "LLM returned no_event for a manual event request".to_string();
        eprintln!("[event] {}", message);
        return Err(message);
    }
    eprintln!("[event] applying type={} summary={}", proposal.event_type, proposal.summary);
    let updated = engine.lock().map_err(|e| e.to_string())?.apply(proposal).map_err(|e| e.to_string())?;
    let _ = app.emit("world-updated", &updated);
    eprintln!("[event] applied successfully");
    Ok(updated)
}

fn normalize_proposal_value(value: &mut Value) {
    let Some(object) = value.as_object_mut() else { return; };
    if let Some(Value::String(text)) = object.get("effects").cloned() {
        object.insert("effects".into(), parse_effect_text(&text));
    }
    if let Some(Value::String(text)) = object.get("memory").cloned() {
        object.insert("memory".into(), Value::Bool(!text.trim().is_empty() && text != "false"));
    } else if object.get("memory").map(Value::is_null).unwrap_or(true) {
        object.insert("memory".into(), Value::Bool(false));
    }
}

fn parse_effect_text(text: &str) -> Value {
    let mut effects = Map::new();
    let labels = [
        ("能量", "energy"), ("心情", "mood"), ("体力", "health"),
        ("智力", "intelligence"), ("好奇心", "curiosity"), ("社交", "friendship"),
        ("创造力", "creativity"), ("勇气", "courage"), ("金币", "money"),
        ("探索度", "exploration"), ("经验", "xp"),
    ];
    for part in text.split(|character| matches!(character, '，' | ',' | '、' | ';' | '；')) {
        let Some((_, key)) = labels.iter().find(|(label, _)| part.contains(label)) else { continue; };
        let number = part.chars().collect::<Vec<_>>().windows(2)
            .enumerate()
            .find_map(|(index, pair)| {
                if (pair[0] == '+' || pair[0] == '-') && pair[1].is_ascii_digit() {
                    Some(index)
                } else { None }
            })
            .or_else(|| part.chars().position(|character| character.is_ascii_digit()));
        let Some(start) = number else { continue; };
        let numeric = part.chars().skip(start)
            .take_while(|character| character.is_ascii_digit() || matches!(character, '+' | '-' | '.'))
            .collect::<String>();
        if let Ok(parsed) = numeric.parse::<f32>() {
            effects.insert((*key).into(), serde_json::json!(parsed));
        }
    }
    Value::Object(effects)
}

pub fn run() {
    tauri::Builder::default()
        .manage(Mutex::new(Engine::open().expect("world storage")))
        .on_window_event(|window, event| {
            match event {
                WindowEvent::Moved(_) => {
                    if window.label() == "pet" {
                        if let Some(pet) = window.app_handle().get_webview_window("pet") {
                            configure_pet_window(&pet);
                        }
                        if let (Ok(position), Some(chronicle)) = (window.outer_position(), window.app_handle().get_webview_window("chronicle")) {
                            let _ = chronicle.set_position(Position::Physical(PhysicalPosition::new(position.x + 245, position.y)));
                        }
                    }
                }
                WindowEvent::Focused(true) if window.label() == "pet" => {
                    if let Some(pet) = window.app_handle().get_webview_window("pet") {
                        configure_pet_window(&pet);
                    }
                }
                _ => {}
            }
        })
        .setup(|app| {
            if let Some(pet) = app.get_webview_window("pet") {
                let _ = pet.set_decorations(false);
                let _ = pet.set_shadow(false);
                let _ = pet.set_background_color(Some(tauri::window::Color(0, 0, 0, 0)));
                configure_pet_window(&pet);
            }
            let toggle = tauri::menu::MenuItemBuilder::with_id("toggle_chronicle", "Show / Hide Chronicle").build(app)?;
            let settings = tauri::menu::MenuItemBuilder::with_id("settings", "Settings").build(app)?;
            let exit = tauri::menu::MenuItemBuilder::with_id("exit", "Exit").build(app)?;
            let menu = tauri::menu::MenuBuilder::new(app).items(&[&toggle, &settings, &exit]).build()?;
            tauri::tray::TrayIconBuilder::new().menu(&menu).on_menu_event(|app, event| match event.id().as_ref() {
                "toggle_chronicle" => {
                    if let Some(window) = app.get_webview_window("chronicle") {
                        if window.is_visible().unwrap_or(false) { let _ = window.hide(); } else { let _ = window.show(); let _ = window.set_focus(); }
                    }
                }
                "settings" => {
                    if let Some(window) = app.get_webview_window("chronicle") {
                        let _ = window.show();
                        let _ = window.set_focus();
                        let _ = app.emit("open-settings", ());
                    }
                }
                "exit" => app.exit(0),
                _ => {}
            }).build(app)?;
            let handle = app.handle().clone();
            std::thread::spawn(move || loop {
                std::thread::sleep(Duration::from_secs(60));
                let Some(state) = handle.try_state::<Mutex<Engine>>() else { continue; };
                let Ok(mut engine) = state.lock() else { continue; };
                if let Err(error) = engine.scheduler_tick() {
                    eprintln!("[scheduler] tick failed: {}", error);
                    continue;
                }
                if let Ok(snapshot) = engine.snapshot() {
                    let _ = handle.emit("world-updated", &snapshot);
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_world, apply_proposal, reset_world, initialize_world, initialize_world_ai, scheduler_tick, set_rest_hours, toggle_chronicle, test_provider, generate_event])
        .run(tauri::generate_context!())
        .expect("error while running The You Beyond");
}

fn main() {
    run();
}
