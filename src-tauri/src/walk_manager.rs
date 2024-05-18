use std::sync::Arc;
use std::sync::Mutex;

use crate::node::Node;
use crate::progress::ErrorHandler;
use crate::progress::ProgressHandler;

// Walkの結果や実行状態を管理するマネージャー
pub struct WalkManager{
    node: Mutex<Option<Node>>,          // ノード格納用
    errors: Arc<Mutex<ErrorHandler>>,   // エラー格納用
    progress: Arc<ProgressHandler>,     // 処理ステータス格納用
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
    pub fn get_full_node(&self) -> Option<Node> {
        let locked_node = self.node.lock().unwrap();
        return locked_node.clone();
    }

    // 深さ指定で部分的ノードを取得
    pub fn get_partial_node(&self, depth: usize) -> Option<Node> {
        let locked_node = self.node.lock().unwrap();

        match locked_node.clone() {
            Some(node) => {
                return Some(create_partial_node(&node, depth));
            },
            None => {
                return None;
            },
        };
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

// 深さ指定で部分的ノードを生成
pub fn create_partial_node(node: &Node, depth: usize) -> Node {
    let mut new_node = Node {
        name: node.name.clone(),
        size: node.size,
        children: Vec::new(),
        inode_device: node.inode_device,
        depth: node.depth,
    };

    if new_node.depth >= depth {
        // Depth exceeds 6, truncate children
        new_node.children.clear();
    } else {
        // Process children recursively
        for child in &node.children {
            new_node.children.push(create_partial_node(child, depth));
        }
    }

    return new_node;
}