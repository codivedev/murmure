#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, menu::Menu, tray::TrayIconBuilder};

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_clipboard::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            murmure2_lib::get_settings,
            murmure2_lib::save_settings,
            murmure2_lib::store_api_key,
            murmure2_lib::has_api_key,
            murmure2_lib::start_audio_recording,
            murmure2_lib::stop_audio_recording,
            murmure2_lib::transcribe_audio,
            murmure2_lib::insert_text,
            murmure2_lib::register_shortcut,
            murmure2_lib::get_active_window,
        ])
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                app.get_webview_window("main").unwrap().open_devtools();
            }
            
            let tray_menu = Menu::with_items(
                app,
                &[
                    &tauri::menu::MenuItem::with_id(app, "settings", "Paramètres", true, None::<&str>)?,
                    &tauri::menu::MenuItem::with_id(app, "quit", "Quitter", true, None::<&str>)?,
                ],
            )?;
            
            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&tray_menu)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "settings" => {
                        if let Some(window) = app.get_webview_window("main") {
                            window.show().unwrap();
                            window.set_focus().unwrap();
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .build(app)?;
                
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}