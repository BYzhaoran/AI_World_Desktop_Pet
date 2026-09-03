#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod world;
mod llm;
mod scheduler;

use std::sync::Mutex;
use std::time::Duration;
use tauri::{Emitter, Manager, Position, PhysicalPosition, State, WindowEvent};
use world::{Engine, EventProposal, WorldSnapshot};
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
fn scheduler_tick(engine: State<'_, Mutex<Engine>>) -> Result<Option<String>, String> {
    engine.lock().map_err(|e| e.to_string())?.scheduler_tick().map_err(|e| e.to_string())
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
async fn generate_event(app: tauri::AppHandle, engine: State<'_, Mutex<Engine>>, base_url: String, model: String, api_key: Option<String>, language: Option<String>, character_context: Option<String>) -> Result<WorldSnapshot, String> {
    let (snapshot, memory, location_due) = {
        let guard = engine.lock().map_err(|e| e.to_string())?;
        let snapshot = guard.snapshot().map_err(|e| e.to_string())?;
        let memory = guard.memory_context();
        let location_due = guard.location_change_due();
        (snapshot, memory, location_due)
    };
    let prompt = format!(
        "Generate one concrete causal world event from the state, memory, and character context below. Return exactly one valid JSON object, with no Markdown or explanation. Required fields: event_type (one of normal_event, social_event, activity_event, weather_event, discovery_event, item_event, skill_event, relationship_event, important_event, milestone_event, level_up), summary (concise Simplified Chinese sentence), importance (number 0..1), location (string), effects (object with optional numeric energy, mood, health, xp, intelligence, curiosity, friendship, creativity, courage), participants (string array), causes (string array), memory (boolean). For a manual event action, do not return no_event. Do not use a fixed example and do not modify state directly. STATE: {}. MEMORY: {}. CHARACTER: {}. Current local time: {}. A location change opportunity is {}. If the opportunity is false, keep the current location exactly. If true, decide whether to move based on the time, current behavior, energy, mood, weather and known locations; if moving, choose a different known location and make the summary describe the travel.",
        serde_json::to_string(&snapshot).map_err(|e| e.to_string())?,
        memory,
        character_context.unwrap_or_default(),
        snapshot.world_time,
        if location_due { "available" } else { "not available" }
    );
    let config = ProviderConfig {
        base_url,
        model,
        api_key: api_key.unwrap_or_default(),
        language: language.unwrap_or_else(|| "zh".into()),
    };
    eprintln!("[event] generating event with configured provider");
    let raw = llm::generate(config, prompt).await?;
    let value = llm::parse_proposal(&raw)?;
    eprintln!("[event] parsed proposal json={}", value);
    let mut proposal = serde_json::from_value::<EventProposal>(value).map_err(|error| {
        let message = format!("LLM event schema mismatch: {}", error);
        eprintln!("[event] {}", message);
        message
    })?;
    if !location_due {
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
        .invoke_handler(tauri::generate_handler![get_world, apply_proposal, reset_world, scheduler_tick, toggle_chronicle, test_provider, generate_event])
        .run(tauri::generate_context!())
        .expect("error while running Aoi's World");
}

fn main() {
    run();
}
