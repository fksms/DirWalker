use std::sync::Arc;
use std::sync::Mutex;

use crate::node::Node;
use crate::progress::RuntimeErrors;
use crate::progress::Progress;

// Walkの結果や実行状態を管理するマネージャー
pub struct WalkManager{
    nodes: Mutex<Option<Vec<Node>>>,        // ノード格納用
    errors: Arc<Mutex<RuntimeErrors>>,      // エラー格納用
    progress: Arc<Progress>,
}

impl WalkManager {
    // 初期化
    pub fn new() -> Self {
        Self {
            nodes: Mutex::new(None),
            errors: Arc::new(Mutex::new(RuntimeErrors::default())),
            progress: Arc::new(Progress::default()),
        }
    }

    // ノードをセット
    pub fn set_nodes(&self, nodes: Option<Vec<Node>>) {
        let mut locked_nodes = self.nodes.lock().unwrap();
        *locked_nodes = nodes;
    }

    /*
    // ノード全体を取得
    pub fn get_all_nodes(&self) -> Option<Vec<Node>> {
        let locked_nodes = self.nodes.lock().unwrap();
        return locked_nodes.clone();
    }
    */

    // 深さ指定で部分的ノードを取得
    pub fn get_partial_nodes(&self, depth: usize) -> Option<Vec<Node>> {
        let locked_nodes = self.nodes.lock().unwrap();

        let new_nodes = match locked_nodes.clone() {
            Some(nodes) => nodes
                .iter()
                .map(|node| create_partial_node(node, depth))
                .collect::<Vec<Node>>(),
            None => {
                return None;
            },
        };

        return Some(new_nodes);
    }

    // errorハンドラを取得
    pub fn get_error_handler(&self) -> &Arc<Mutex<RuntimeErrors>> {
        return &(self.errors);
    }

    // 終了フラグを設定
    pub fn set_abort_flag(&self, flag: bool) {
        let mut locked_errors = self.errors.lock().unwrap();
        locked_errors.abort = flag;
    }

    // progressハンドラを取得
    pub fn get_progress_handler(&self) -> &Arc<Progress> {
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