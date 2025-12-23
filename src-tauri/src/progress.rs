use serde::Serialize;
use std::{
    collections::HashSet,
    sync::{
        atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering},
        mpsc::{self, RecvTimeoutError, Sender},
        Arc,
    },
    thread::JoinHandle,
    time::Duration,
};

use tauri::Emitter;

// インジケーターの更新間隔
const INDICATOR_UPDATE_INTERVAL: u64 = 100; // [ms]

pub const ORDERING: Ordering = Ordering::Relaxed;

/* -------------------------------------------------------------------------- */

#[derive(Default, Serialize)]
pub struct ProgressHandler {
    pub num_files: AtomicUsize,
    pub total_file_size: AtomicU64,
    pub scan_complete: AtomicBool,
}

impl ProgressHandler {
    pub fn clear_state(&self) {
        self.total_file_size.store(0, ORDERING);
        self.num_files.store(0, ORDERING);
        self.scan_complete.store(false, ORDERING);
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
        // While the timeout triggers we go round the loop
        // If we disconnect or the sender sends its message we exit the while loop
        while let Err(RecvTimeoutError::Timeout) =
            receiver.recv_timeout(Duration::from_millis(INDICATOR_UPDATE_INTERVAL))
        {
            let encode_result: Result<String, _> = serde_json::to_string(&prog_data);
            match encode_result {
                // 正常にエンコードできた場合はWebViewに送信
                Ok(str) => app.emit("ProgressNotification", str).unwrap(),
                // エンコードに失敗した場合
                Err(err) => eprintln!("Progress encode error: {}", err),
            }
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
