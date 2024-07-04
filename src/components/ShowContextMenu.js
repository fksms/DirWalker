import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";


// コンテキストメニューを表示する関数
async function showContextMenu(node) {

    // イベントを受信するためのリスナーを起動
    const unlisten = await listen("openFileManager", event => {
        // ファイルマネージャーを開く
        openFileManager(event.payload);
        // リスナーを停止
        unlisten();
    });

    // バックエンド側の関数を実行
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