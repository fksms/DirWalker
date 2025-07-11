import { invoke } from '@tauri-apps/api/core';
import { Menu, MenuItem } from '@tauri-apps/api/menu';
import { ask, message } from '@tauri-apps/plugin-dialog';
import { writeText } from '@tauri-apps/plugin-clipboard-manager';
//import { revealItemInDir } from "@tauri-apps/plugin-opener"

import i18n from './i18n';

// コンテキストメニューを表示する関数
async function showContextMenu(node, callback) {
    // メニューアイテムの生成
    const menuItems = [
        await MenuItem.new({
            text: i18n.global.t('context_menu.copy_path'),
            action: async () => {
                await writeToClipboard(node.data.name);
            },
        }),
        await MenuItem.new({
            text: i18n.global.t('context_menu.open'),
            action: async () => {
                await openFileManager(node.children ? node.data.name : node.parent.data.name);
            },
        }),
        await MenuItem.new({
            text: i18n.global.t('context_menu.move_to_trash'),
            action: async () => {
                await moveToTrash(node.data.name, node, callback);
            },
        }),
    ];

    // メニューの生成
    const menu = await Menu.new({ items: menuItems });

    // メニューをポップアップ
    menu.popup();
}

// クリップボードに書き込む関数
async function writeToClipboard(path) {
    await writeText(path);
}

// ファイルマネージャーを開く関数
async function openFileManager(path) {
    //await revealItemInDir(path);
    await invoke('open_file_manager', { path: path })
        // 失敗した場合
        .catch((failure) => {
            // Message Dialog
            message(failure);
        });
}

// ファイル or ディレクトリを削除する関数
async function removeFileOrDirectory(path, node, callback) {
    let dialogTitle = '';
    let dialogMessage = '';

    if (node.children) {
        dialogTitle = i18n.global.t('removal_alert.directory');
        dialogMessage = i18n.global.t('removal_alert.directory_desc') + '\n\n\n' + path + '\n';
    } else {
        dialogTitle = i18n.global.t('removal_alert.file');
        dialogMessage = i18n.global.t('removal_alert.file_desc') + '\n\n\n' + path + '\n';
    }

    const result = await ask(dialogMessage, dialogTitle);
    // YESの場合
    if (result) {
        // バックエンド側の関数を実行
        await invoke('remove_file_or_directory', { path: path })
            // 成功した場合
            .then((success) => {
                // Nodeを削除
                callback(node);
            })
            // 失敗した場合
            .catch((failure) => {
                // Message Dialog
                message(failure);
            });
    }
}

// ゴミ箱に移動する関数
async function moveToTrash(path, node, callback) {
    let dialogTitle = '';
    let dialogMessage = '';

    if (node.children) {
        dialogTitle = i18n.global.t('removal_alert.directory');
        dialogMessage = i18n.global.t('removal_alert.directory_desc') + '\n\n\n' + path + '\n';
    } else {
        dialogTitle = i18n.global.t('removal_alert.file');
        dialogMessage = i18n.global.t('removal_alert.file_desc') + '\n\n\n' + path + '\n';
    }

    const result = await ask(dialogMessage, dialogTitle);
    // YESの場合
    if (result) {
        // バックエンド側の関数を実行
        await invoke('move_to_trash', { path: path })
            // 成功した場合
            .then((success) => {
                // Nodeを削除
                callback(node);
            })
            // 失敗した場合
            .catch((failure) => {
                // Message Dialog
                message(failure);
            });
    }
}

// 外部に公開
export { showContextMenu };
