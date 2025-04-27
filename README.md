# Input Tracker

A lightweight desktop application that logs mouse movements, clicks, and keyboard inputs to a CSV file. Built with Tauri, React, and TypeScript.


## Features

- **Background Tracking**: Runs in the system tray with minimal resource usage
- **Comprehensive Logging**: Records all mouse movements, clicks, and keyboard inputs
- **CSV Export**: Saves all input data with timestamps in a structured format
- **Minimalistic UI**: Simple interface showing tracking status and log file location


## Installation

### Prerequisites

- [Node.js](https://nodejs.org/) (version 16 or higher)
- [Rust](https://www.rust-lang.org/tools/install) (version 1.64 or higher)

### Setup

1. Clone the repository
   ```bash
   git clone https://github.com/vagxrth/dockable-app.git
   ```

2. Install dependencies
   ```bash
   pnpm install
   ```

3. Run in development mode
   ```bash
   pnpm tauri dev
   ```

4. Build for production
   ```bash
   pnpm tauri build
   ```

## Usage

1. Launch the application
2. The app will automatically start tracking inputs and display status information
3. The app minimizes to the system tray when closed
4. Click the tray icon to show/hide the window
5. Right-click the tray icon for additional options

## Log File Format

The application logs data to `input_log.csv` with the following format:

```
timestamp,event_type,sub_type,data
2023-06-15T12:30:45.123456Z,mouse,mouse_move,{"location":[1024,768]}
2023-06-15T12:30:46.234567Z,mouse,mouse_down,{"button":0}
2023-06-15T12:30:47.345678Z,keyboard,key_down,{"key":"A"}
```

## Demo

Check out the application in action:

https://github.com/user-attachments/assets/e24fb2ec-0d76-45e2-b644-24dc4a771a24

