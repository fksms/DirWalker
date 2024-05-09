use std::sync::Mutex;

use crate::node::Node;

// Walkの結果や実行状態を管理するマネージャー
pub struct WalkManager{
    nodes: Mutex<Option<Vec<Node>>>,    // ノード格納用
    abort_flag: Mutex<bool>,            // 強制終了フラグ
}

impl WalkManager {
    // 初期化
    pub fn new() -> Self {
        Self {
            nodes: Mutex::new(None),
            abort_flag: Mutex::new(false)
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

    // 強制終了フラグをセット
    pub fn set_abort_flag(&self, flag: bool) {
        let mut locked_abort_flag = self.abort_flag.lock().unwrap();
        *locked_abort_flag = flag;
    }

    // 強制終了フラグ状態を取得
    pub fn get_abort_flag(&self) -> bool {
        let locked_abort_flag = self.abort_flag.lock().unwrap();
        return locked_abort_flag.clone();
    }
}