import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";


// リスナー関数
async function registerListeners() {
    await listen("openFileManager", event => {
        openFileManager(event.payload);
    });
}
registerListeners(); // リスナーを起動


// コンテキストメニューを表示する関数
async function showContextMenu(node) {
    await invoke("plugin:context_menu|show_context_menu", {
        items: [
            {
                label: "Open",
                disabled: false,
                event: "openFileManager",
                payload: (node.children) ? node.data.name : node.parent.data.name,
            }
        ],
    });
}


// ファイルマネージャーを開く関数
async function openFileManager(path) {
    await invoke("open_file_manager", { path: path })
        // 失敗した場合
        .catch((failure) => {
            // エラーメッセージを出力
            console.error(failure);
        });
}


// 外部に公開
export { showContextMenu }