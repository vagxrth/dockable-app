use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        div {
            style: "padding: 2rem; max-width: 800px; margin: 0 auto; font-family: system-ui, -apple-system, sans-serif;",
            h1 {
                style: "color: #2C3E50;",
                "Input Tracker"
            }
            div {
                style: "background: #F8F9FA; padding: 1rem; border-radius: 8px; margin-top: 1rem;",
                p {
                    style: "color: #2C3E50;",
                    strong { "Status: " }
                    "Tracking input..."
                }
                p {
                    style: "color: #2C3E50;",
                    strong { "Log File: " }
                    "input_log.csv"
                }
            }
            div {
                style: "margin-top: 2rem; padding: 1rem; background: #E9ECEF; border-radius: 8px;",
                h2 {
                    style: "color: #2C3E50; font-size: 1.2rem;",
                    "About"
                }
                p {
                    style: "color: #2C3E50;",
                    "This application tracks mouse movements, clicks, and keyboard input. All events are logged to a CSV file with timestamps."
                }
            }
        }
    }
} 