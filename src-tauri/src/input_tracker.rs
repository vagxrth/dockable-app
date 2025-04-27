use chrono::Utc;
use device_query::{DeviceQuery, DeviceState, MouseState, Keycode};
use std::fs::{OpenOptions, File};
use std::io::Write;
use std::sync::Mutex;
use std::time::Duration;
use tokio::time;

pub struct InputTracker {
    log_file: Mutex<File>,
    device_state: DeviceState,
    last_mouse_pos: Mutex<(i32, i32)>,
}

impl InputTracker {
    pub fn new(log_file_path: &str) -> Self {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_file_path)
            .expect("Failed to open log file");

        Self {
            log_file: Mutex::new(file),
            device_state: DeviceState::new(),
            last_mouse_pos: Mutex::new((0, 0)),
        }
    }

    fn log_event(&self, event_type: &str, sub_type: &str, data: &str) {
        let now = Utc::now();
        let mut file = self.log_file.lock().unwrap();
        writeln!(
            file,
            "{},{},{},{}",
            now.to_rfc3339_opts(chrono::SecondsFormat::Micros, true),
            event_type,
            sub_type,
            data
        ).expect("Failed to write to log file");
    }

    pub async fn start_tracking(&self) {
        loop {
            // Track mouse movement
            let mouse: MouseState = self.device_state.get_mouse();
            let current_pos = mouse.coords;
            let mut last_pos = self.last_mouse_pos.lock().unwrap();
            
            if current_pos != *last_pos {
                self.log_event(
                    "mouse",
                    "mouse_move",
                    &format!("{{\"location\":{:?}}}", current_pos)
                );
                *last_pos = current_pos;
            }

            // Track mouse buttons
            for (button_idx, pressed) in mouse.button_pressed.iter().enumerate() {
                if *pressed {
                    self.log_event(
                        "mouse",
                        "mouse_down",
                        &format!("{{\"button\":{}}}", button_idx)
                    );
                }
            }

            // Track keyboard
            let keys: Vec<Keycode> = self.device_state.get_keys();
            for key in keys {
                self.log_event(
                    "keyboard",
                    "key_down",
                    &format!("{{\"key\":\"{:?}\"}}", key)
                );
            }

            time::sleep(Duration::from_millis(50)).await;
        }
    }
} 