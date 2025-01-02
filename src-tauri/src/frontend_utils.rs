//use tauri::menu::Menu;

use std::{fs, path::Path};

// ファイルorディレクトリを削除する
#[tauri::command(rename_all = "snake_case")]
pub async fn remove_file_or_directory(path: String) -> Result<(), String> {
    // ディレクトリの場合
    if Path::new(&path).is_dir() {
        match fs::remove_dir_all(path) {
            Ok(ok) => Ok(ok),
            Err(err) => Err(err.to_string()),
        }
    }
    // ファイルの場合
    else {
        match fs::remove_file(path) {
            Ok(ok) => Ok(ok),
            Err(err) => Err(err.to_string()),
        }
    }
}

/*
// メニューを生成する
pub fn generate_menu(app_name: String) -> Menu {
    #[allow(unused_mut)]
    let mut menu = Menu::new();

    let menus = Menu::with_items(app, &[&visible])?;

    #[cfg(not(target_os = "windows"))]
    {
        use tauri::menu::{MenuItem, Submenu};

        #[cfg(target_os = "macos")]
        {
            use tauri::menu::AboutMetadata;

            menu = menu.add_submenu(Submenu::new(
                app_name.clone(), // アプリケーション名
                Menu::new()
                    .add_native_item(MenuItem::About(
                        app_name, // アプリケーション名
                        AboutMetadata::default(),
                    ))
                    .add_native_item(MenuItem::Separator)
                    .add_native_item(MenuItem::Services)
                    .add_native_item(MenuItem::Separator)
                    .add_native_item(MenuItem::Hide)
                    .add_native_item(MenuItem::HideOthers)
                    .add_native_item(MenuItem::ShowAll)
                    .add_native_item(MenuItem::Separator)
                    .add_native_item(MenuItem::Quit),
            ));
        }

        #[cfg(target_os = "linux")]
        {
            let mut file_menu = Menu::new();
            file_menu = file_menu.add_native_item(MenuItem::Quit);
            menu = menu.add_submenu(Submenu::new("File", file_menu));
        }

        #[cfg(target_os = "macos")]
        {
            let mut edit_menu = Menu::new();
            /*
            edit_menu = edit_menu.add_native_item(MenuItem::Undo);
            edit_menu = edit_menu.add_native_item(MenuItem::Redo);
            edit_menu = edit_menu.add_native_item(MenuItem::Separator);
            */
            edit_menu = edit_menu.add_native_item(MenuItem::Cut);
            edit_menu = edit_menu.add_native_item(MenuItem::Copy);
            edit_menu = edit_menu.add_native_item(MenuItem::Paste);
            edit_menu = edit_menu.add_native_item(MenuItem::SelectAll);
            menu = menu.add_submenu(Submenu::new("Edit", edit_menu));

            /*
            menu = menu.add_submenu(Submenu::new(
                "View",
                Menu::new().add_native_item(MenuItem::EnterFullScreen),
            ));
            */
        }

        let mut window_menu = Menu::new();
        window_menu = window_menu.add_native_item(MenuItem::Minimize);
        /*
        #[cfg(target_os = "macos")]
        {
            window_menu = window_menu.add_native_item(MenuItem::Zoom);
        }
        */
        window_menu = window_menu.add_native_item(MenuItem::Separator);
        window_menu = window_menu.add_native_item(MenuItem::CloseWindow);

        menu = menu.add_submenu(Submenu::new("Window", window_menu));
    }

    return menu;
}
*/
