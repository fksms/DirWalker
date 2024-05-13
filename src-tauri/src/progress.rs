use std::{
    collections::HashSet, io::Write, sync::{
        atomic::{AtomicU64, AtomicUsize, Ordering},
        mpsc::{self, RecvTimeoutError, Sender},
        Arc,
    }, thread::JoinHandle, time::Duration
};

pub const ORDERING: Ordering = Ordering::Relaxed;

/* -------------------------------------------------------------------------- */

#[derive(Default)]
pub struct Progress {
    pub num_files: AtomicUsize,
    pub total_file_size: AtomicU64,
}

impl Progress {
    pub fn clear_state(&self) {
        self.total_file_size.store(0, ORDERING);
        self.num_files.store(0, ORDERING);
    }
}

#[derive(Default)]
pub struct RuntimeErrors {
    pub no_permissions: bool,
    pub file_not_found: HashSet<String>,
    pub unknown_error: HashSet<String>,
    pub abort: bool,
}

/* -------------------------------------------------------------------------- */

// Progressを表示
pub fn indicator_spawn(progress: &Arc<Progress>) -> Option<(Sender<()>, JoinHandle<()>)> {
    let prog_data = progress.clone();
    
    let (stop_handler, receiver) = mpsc::channel::<()>();
    
    let time_info_thread = std::thread::spawn(move || {
        let mut stdout = std::io::stdout();
        let mut msg = "".to_string();

        // While the timeout triggers we go round the loop
        // If we disconnect or the sender sends its message we exit the while loop
        while let Err(RecvTimeoutError::Timeout) =
            receiver.recv_timeout(Duration::from_millis(100))
        {
            // Clear the text written by 'write!'& Return at the start of line
            print!("\r{:width$}", " ", width = msg.len());

            let file_count = prog_data.num_files.load(ORDERING);
            let size = prog_data.total_file_size.load(ORDERING);
            let file_str = format!("{file_count} files, {size}");
            msg = format!("Indexing: {file_str}");

            write!(stdout, "\r{msg}").unwrap();
            stdout.flush().unwrap();
        }
    });

    return Some((stop_handler, time_info_thread));
}

// Progressを終了
pub fn indicator_stop(thread: Option<(Sender<()>, JoinHandle<()>)>) {
    match thread {
        Some(t) => {
            t.0.send(()).unwrap();
            t.1.join().unwrap();
        }
        None => {}
    }
}