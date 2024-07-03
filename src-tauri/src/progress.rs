use std::{
    collections::HashSet,
    io::Write,
    sync::{
        atomic::{AtomicU64, AtomicUsize, Ordering},
        mpsc::{self, RecvTimeoutError, Sender},
        Arc,
    },
    thread::JoinHandle,
    time::Duration,
};

use tauri::Manager;

// インジケーターの更新間隔
const INDICATOR_UPDATE_INTERVAL: u64 = 100; // [ms]

pub const ORDERING: Ordering = Ordering::Relaxed;

/* -------------------------------------------------------------------------- */

#[derive(Default)]
pub struct ProgressHandler {
    pub num_files: AtomicUsize,
    pub total_file_size: AtomicU64,
}

impl ProgressHandler {
    pub fn clear_state(&self) {
        self.total_file_size.store(0, ORDERING);
        self.num_files.store(0, ORDERING);
    }
}

#[derive(Default)]
pub struct ErrorHandler {
    pub no_permissions: bool,
    pub file_not_found: HashSet<String>,
    pub unknown_error: HashSet<String>,
    pub abort: bool,
}

/* -------------------------------------------------------------------------- */

// Progressを表示
pub fn indicator_spawn(
    progress: &Arc<ProgressHandler>,
    app: tauri::AppHandle,
) -> Option<(Sender<()>, JoinHandle<()>)> {
    let prog_data = progress.clone();

    let (sender, receiver) = mpsc::channel::<()>();

    let indicator_thread = std::thread::spawn(move || {
        let mut stdout = std::io::stdout();
        let mut msg = "".to_string();

        // While the timeout triggers we go round the loop
        // If we disconnect or the sender sends its message we exit the while loop
        while let Err(RecvTimeoutError::Timeout) =
            receiver.recv_timeout(Duration::from_millis(INDICATOR_UPDATE_INTERVAL))
        {
            // Clear the text written by 'write!'& Return at the start of line
            print!("\r{:width$}", " ", width = msg.len());

            let file_count = prog_data.num_files.load(ORDERING);
            let size_count = prog_data.total_file_size.load(ORDERING);
            msg = format!("Scanned:  {file_count} files,  {size_count} bytes");

            // WebViewに送信
            app.emit_all("ProgressNotification", msg.clone()).unwrap();

            write!(stdout, "\r{msg}").unwrap();
            stdout.flush().unwrap();
        }
    });

    return Some((sender, indicator_thread));
}

// Progressを終了
pub fn indicator_stop(thread: Option<(Sender<()>, JoinHandle<()>)>) {
    match thread {
        Some(t) => {
            t.0.send(()).unwrap(); // sendすることでreceiverを停止させる
            t.1.join().unwrap();
        }
        None => {}
    }
}
