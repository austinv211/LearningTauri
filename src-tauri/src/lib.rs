use serde::{Deserialize, Serialize};
use sysinfo::{Process, ProcessStatus, System};

#[cfg(target_os = "windows")]
const APPLICATION_DIRS: &[&str] = &["C:\\Program Files", "C:\\Program Files (x86)"];

#[cfg(target_os = "macos")]
const APPLICATION_DIRS: &[&str] = &["/Applications", "/Users/*/Applications"];

#[cfg(target_os = "linux")]
const APPLICATION_DIRS: &[&str] = &["/usr/bin", "/usr/local/bin", "/opt"];

#[derive(Serialize, Deserialize, Debug)]
struct AppInfo {
    id: String,
    name: String,
    running_time_formatted: String,
    memory_in_bytes: u64,
}

fn is_valid(process: &Process) -> bool {
    let exe = process.exe();
    match exe {
        None => {
            return false;
        }
        Some(value) => {
            let exe_path = value.to_str().unwrap_or("");
            if exe_path == "" {
                return false;
            }
            let is_in_app_dir = APPLICATION_DIRS.iter().any(|dir| exe_path.starts_with(dir));
            return process.status() == ProcessStatus::Run && is_in_app_dir;
        }
    }
}

fn format_running_time(run_time: u64) -> String {
    const HOUR: u64 = 3600;
    const DAY: u64 = 24 * HOUR;
    const MINUTE: u64 = 60;

    let mut n = run_time;
    let days = n / DAY;

    n = n % DAY;
    let hours = n / HOUR;

    n = n % HOUR;
    let minutes = n / 60;

    n = n % MINUTE;
    let seconds = n;

    format!(
        "{:02} Days : {:02} Hours : {:02} Minutes : {:02} Seconds",
        days, hours, minutes, seconds
    )
}

#[tauri::command]
fn max_running_process() -> Option<AppInfo> {
    let mut sys = System::new_all();
    sys.refresh_all();

    sys.processes()
        .iter()
        .filter(|(_, process)| is_valid(process))
        .max_by_key(|(_, process)| process.run_time())
        .map(|(id, process)| AppInfo {
            id: id.to_string(),
            name: process.name().to_string_lossy().into_owned(),
            running_time_formatted: format_running_time(process.run_time()),
            memory_in_bytes: process.memory(),
        })
}

#[tauri::command]
fn max_memory() -> Option<AppInfo> {
    let mut sys = System::new_all();
    sys.refresh_all();

    sys.processes()
        .iter()
        .filter(|(_, process)| is_valid(process))
        .max_by_key(|(_, process)| process.memory())
        .map(|(id, process)| AppInfo {
            id: id.to_string(),
            name: process.name().to_string_lossy().into_owned(),
            running_time_formatted: format_running_time(process.run_time()),
            memory_in_bytes: process.memory(),
        })
}

#[tauri::command]
fn list_process() -> Vec<AppInfo> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let mut processes: Vec<AppInfo> = sys
        .processes()
        .iter()
        .filter(|(_, process)| is_valid(process))
        .map(|(id, process)| AppInfo {
            id: id.to_string(),
            name: process.name().to_string_lossy().into_owned(),
            running_time_formatted: format_running_time(process.run_time()),
            memory_in_bytes: process.memory(),
        })
        .collect();

    processes.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    processes
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            list_process,
            max_memory,
            max_running_process
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
