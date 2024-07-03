use std::sync::Arc;
use std::sync::Mutex;

use crate::node::Node;
use crate::progress::ErrorHandler;
use crate::progress::ProgressHandler;

// Walkの結果や実行状態を管理するマネージャー
pub struct WalkManager {
    node: Mutex<Option<Node>>,        // ノード格納用
    errors: Arc<Mutex<ErrorHandler>>, // エラー格納用
    progress: Arc<ProgressHandler>,   // 処理ステータス格納用
}

impl WalkManager {
    // 初期化
    pub fn new() -> Self {
        Self {
            node: Mutex::new(None),
            errors: Arc::new(Mutex::new(ErrorHandler::default())),
            progress: Arc::new(ProgressHandler::default()),
        }
    }

    // ノードをセット
    pub fn set_node(&self, node: Option<Node>) {
        let mut locked_node = self.node.lock().unwrap();
        *locked_node = node;
    }

    // ノード全体を取得
    pub fn get_node(&self) -> Option<Node> {
        let locked_node = self.node.lock().unwrap();
        return locked_node.clone();
    }

    // errorハンドラを取得
    pub fn get_error_handler(&self) -> &Arc<Mutex<ErrorHandler>> {
        return &(self.errors);
    }

    // 終了フラグを設定
    pub fn set_abort_flag(&self, flag: bool) {
        let mut locked_errors = self.errors.lock().unwrap();
        locked_errors.abort = flag;
    }

    // progressハンドラを取得
    pub fn get_progress_handler(&self) -> &Arc<ProgressHandler> {
        return &(self.progress);
    }
}
