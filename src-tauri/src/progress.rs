use std::{
    collections::HashSet,
    path::Path,
    sync::{
        atomic::{AtomicU64, AtomicU8, AtomicUsize, Ordering},
        mpsc::Sender,
        Arc, RwLock,
    },
    thread::JoinHandle,
};

/* -------------------------------------------------------------------------- */

pub const ORDERING: Ordering = Ordering::Relaxed;

pub trait ThreadSyncTrait<T> {
    fn set(&self, val: T);
    fn get(&self) -> T;
}

#[derive(Default)]
pub struct ThreadStringWrapper {
    inner: RwLock<String>,
}

impl ThreadSyncTrait<String> for ThreadStringWrapper {
    fn set(&self, val: String) {
        *self.inner.write().unwrap() = val;
    }

    fn get(&self) -> String {
        (*self.inner.read().unwrap()).clone()
    }
}

/* -------------------------------------------------------------------------- */

// creating an enum this way allows to have simpler syntax compared to a Mutex or a RwLock
#[allow(non_snake_case)]
pub mod Operation {
    pub const INDEXING: u8 = 0;
    pub const PREPARING: u8 = 1;
}

#[derive(Default)]
pub struct PAtomicInfo {
    pub num_files: AtomicUsize,
    pub total_file_size: AtomicU64,
    pub state: AtomicU8,
    pub current_path: ThreadStringWrapper,
}

impl PAtomicInfo {
    pub fn clear_state(&self, dir: &Path) {
        self.state.store(Operation::INDEXING, ORDERING);
        let dir_name = dir.to_string_lossy().to_string();
        self.current_path.set(dir_name);
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

pub struct PIndicator {
    pub thread: Option<(Sender<()>, JoinHandle<()>)>,
    pub data: Arc<PAtomicInfo>,
}

impl PIndicator {
    pub fn build_me() -> Self {
        Self {
            thread: None,
            data: Arc::new(PAtomicInfo {
                ..Default::default()
            }),
        }
    }
}
