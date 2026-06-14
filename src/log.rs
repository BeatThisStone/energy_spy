use std::io::Write;
use std::{
    env,
    fs::{OpenOptions, create_dir_all},
    path::PathBuf,
    sync::Mutex,
};

static LOG_MUTEX: Mutex<()> = Mutex::new(());

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        crate::log::debug_log_internal(&format!($($arg)*));
    };
}

pub fn debug_log_internal(msg: &str) {
    let _guard = LOG_MUTEX.lock().ok();

    let plugin_dir = env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."));
    let log_dir = plugin_dir.join("hachimi");
    let _ = create_dir_all(&log_dir);
    let log_path = log_dir.join("energy_spy.log");

    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(log_path) {
        let _ = writeln!(file, "{}", msg);
    }
}
