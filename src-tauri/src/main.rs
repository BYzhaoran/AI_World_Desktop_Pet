#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod world;
use std::sync::Mutex;
use tauri::State;
use world::{Engine, EventProposal, WorldSnapshot};

#[tauri::command]
fn get_world(engine: State<'_, Mutex<Engine>>) -> Result<WorldSnapshot, String> { engine.lock().map_err(|e|e.to_string())?.snapshot().map_err(|e|e.to_string()) }
#[tauri::command]
fn apply_proposal(engine: State<'_, Mutex<Engine>>, proposal: EventProposal) -> Result<WorldSnapshot, String> { let mut guard=engine.lock().map_err(|e|e.to_string())?; guard.apply(proposal).map_err(|e|e.to_string()) }
#[tauri::command]
fn test_provider(base_url: String, model: String) -> Result<String, String> { if base_url.trim().is_empty() || model.trim().is_empty() { Err("Base URL and model are required".into()) } else { Ok("Provider configuration accepted; API key is never logged.".into()) } }

pub fn run() { tauri::Builder::default().manage(Mutex::new(Engine::open().expect("world storage"))).invoke_handler(tauri::generate_handler![get_world,apply_proposal,test_provider]).run(tauri::generate_context!()).expect("error while running Aoi's World"); }
fn main() { run(); }
