import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";
import { writeText } from '@tauri-apps/api/clipboard';


// コンテキストメニューを表示する関数
async function showContextMenu(node) {

    // イベントを受信するためのリスナーを起動
    const unlisten1 = await listen("writeToClipboard", event => {
        // クリップボードに書き込む
        writeToClipboard(event.payload);
        // リスナーをまとめて停止
        unlistenAll();
    });

    // イベントを受信するためのリスナーを起動
    const unlisten2 = await listen("openFileManager", event => {
        // ファイルマネージャーを開く
        openFileManager(event.payload);
        // リスナーをまとめて停止
        unlistenAll();
    });

    // バックエンド側の関数を実行
    await invoke("plugin:context_menu|show_context_menu", {
        items: [
            {
                label: "Copy path",
                disabled: false,
                event: "writeToClipboard",
                payload: node.data.name,
            },
            {
                label: "Open",
                disabled: false,
                event: "openFileManager",
                payload: (node.children) ? node.data.name : node.parent.data.name,
            }
        ],
    });

    // リスナーをまとめて停止
    function unlistenAll() {
        unlisten1();
        unlisten2();
    }
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


// クリップボードに書き込む関数
async function writeToClipboard(path) {
    await writeText(path);
}


// 外部に公開
export { showContextMenu }