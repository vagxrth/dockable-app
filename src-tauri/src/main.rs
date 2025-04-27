// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod input_tracker;
use input_tracker::InputTracker;
use tauri::{
    AppHandle, Manager, 
    menu::{Menu, MenuItem, IsMenuItem},
    tray::{TrayIconBuilder, TrayIconEvent}
};
use tauri::image::Image;

fn main() {
    let log_file_path = "input_log.csv";
    let tracker = InputTracker::new(log_file_path);
    let tracker_clone = tracker;

    let mut app = tauri::Builder::default()
        .setup(move |app| {
            let app_handle = app.handle();
            
            std::thread::spawn(move || {
                let rt = tokio::runtime::Runtime::new().unwrap();
                rt.block_on(async {
                    tracker_clone.start_tracking().await;
                });
            });

            let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let show_item = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;

            let tray_menu = Menu::with_items(app, &[
                &show_item as &dyn IsMenuItem<_>,
                &quit_item as &dyn IsMenuItem<_>
            ])?;

            let icon = Image::from_path("icons/icon.png")?;
            let app_handle_clone = app_handle.clone();
            
            let _tray_icon = TrayIconBuilder::new()
                .icon(icon)
                .menu(&tray_menu)
                .on_menu_event(move |app_handle, event| {
                    if event.id.as_ref() == "quit" {
                        std::process::exit(0);
                    } else if event.id.as_ref() == "show" {
                        toggle_window(app_handle);
                    }
                })
                .on_tray_icon_event(move |_tray, event| {
                    match event {
                        TrayIconEvent::Click { .. } => {
                            toggle_window(&app_handle_clone);
                        }
                        _ => {}
                    }
                })
                .build(app)?;

            Ok(())
        })
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                window.hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application");
    
    #[cfg(target_os = "macos")]
    app.set_activation_policy(tauri::ActivationPolicy::Accessory);
    
    app.run(|_app_handle, _event| {});
}

fn toggle_window(app: &AppHandle) {
    let window = app.get_webview_window("main").unwrap();
    if window.is_visible().unwrap() {
        window.hide().unwrap();
    } else {
        window.show().unwrap();
        window.set_focus().unwrap();
    }
}
