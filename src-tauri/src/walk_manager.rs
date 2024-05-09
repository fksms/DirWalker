use std::sync::Arc;
use std::sync::Mutex;

use crate::node::Node;
use crate::progress::RuntimeErrors;

// Walkの結果や実行状態を管理するマネージャー
pub struct WalkManager{
    nodes: Mutex<Option<Vec<Node>>>,        // ノード格納用
    errors: Arc<Mutex<RuntimeErrors>>,      // エラー格納用
}

impl WalkManager {
    // 初期化
    pub fn new() -> Self {
        Self {
            nodes: Mutex::new(None),
            errors: Arc::new(Mutex::new(RuntimeErrors::default()))
        }
    }

    // ノードをセット
    pub fn set_nodes(&self, nodes: Option<Vec<Node>>) {
        let mut locked_nodes = self.nodes.lock().unwrap();
        *locked_nodes = nodes;
    }

    // ノードを取得
    pub fn get_nodes(&self) -> Option<Vec<Node>> {
        let locked_nodes = self.nodes.lock().unwrap();
        return locked_nodes.clone();
    }

    // エラーハンドラを取得
    pub fn get_error_handler(&self) -> &Arc<Mutex<RuntimeErrors>> {
        return &(self.errors);
    }

    // 終了フラグを設定
    pub fn set_abort_flag(&self, flag: bool) {
        let mut locked_errors = self.errors.lock().unwrap();
        locked_errors.abort = flag;
    }
}