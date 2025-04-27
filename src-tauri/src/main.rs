// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod input_tracker;
use input_tracker::InputTracker;

fn main() {
    let log_file_path = "input_log.csv";
    let tracker = InputTracker::new(log_file_path);
    let tracker_clone = tracker;

    tauri::Builder::default()
        .setup(move |app| {
            let _app_handle = app.handle();
            
            // Start input tracking in a background task
            std::thread::spawn(move || {
                let rt = tokio::runtime::Runtime::new().unwrap();
                rt.block_on(async {
                    tracker_clone.start_tracking().await;
                });
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
