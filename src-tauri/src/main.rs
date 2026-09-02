#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod world;
mod llm;
mod scheduler;
use std::sync::Mutex;
use tauri::{Manager, State};
use world::{Engine, EventProposal, WorldSnapshot};
use llm::ProviderConfig;

#[tauri::command]
fn get_world(engine: State<'_, Mutex<Engine>>) -> Result<WorldSnapshot, String> { engine.lock().map_err(|e|e.to_string())?.snapshot().map_err(|e|e.to_string()) }
#[tauri::command]
fn apply_proposal(engine: State<'_, Mutex<Engine>>, proposal: EventProposal) -> Result<WorldSnapshot, String> { let mut guard=engine.lock().map_err(|e|e.to_string())?; guard.apply(proposal).map_err(|e|e.to_string()) }
#[tauri::command]
fn reset_world(engine: State<'_, Mutex<Engine>>) -> Result<(), String> { engine.lock().map_err(|e|e.to_string())?.reset().map_err(|e|e.to_string()) }
#[tauri::command]
async fn test_provider(base_url: String, model: String, api_key: Option<String>) -> Result<String, String> { llm::test_connection(ProviderConfig { base_url, model, api_key: api_key.unwrap_or_default(), language: "zh".into() }).await }
#[tauri::command]
async fn generate_event(base_url: String, model: String, api_key: Option<String>, prompt: String, language: Option<String>) -> Result<EventProposal, String> { let raw = llm::generate(ProviderConfig { base_url, model, api_key: api_key.unwrap_or_default(), language: language.unwrap_or_else(|| "zh".into()) }, prompt).await?; let value=llm::parse_proposal(&raw)?; serde_json::from_value(value).map_err(|_| "LLM JSON did not match EventProposal".into()) }

pub fn run() { tauri::Builder::default().manage(Mutex::new(Engine::open().expect("world storage"))).setup(|app| { let show = tauri::menu::MenuItemBuilder::with_id("show", "Show Aoi's World").build(app)?; let exit = tauri::menu::MenuItemBuilder::with_id("exit", "Exit").build(app)?; let menu = tauri::menu::MenuBuilder::new(app).items(&[&show, &exit]).build()?; tauri::tray::TrayIconBuilder::new().menu(&menu).on_menu_event(|app, event| match event.id().as_ref() { "show" => { if let Some(window) = app.get_webview_window("main") { let _ = window.show(); let _ = window.set_focus(); } }, "exit" => app.exit(0), _ => {} }).build(app)?; std::thread::spawn(|| loop { std::thread::sleep(std::time::Duration::from_secs(30)); if let Ok(mut engine) = Engine::open() { let _ = engine.scheduler_tick(); } }); Ok(()) }).invoke_handler(tauri::generate_handler![get_world,apply_proposal,reset_world,test_provider,generate_event]).run(tauri::generate_context!()).expect("error while running Aoi's World"); }
fn main() { run(); }
