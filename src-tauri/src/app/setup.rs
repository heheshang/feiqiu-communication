use tauri::{AppHandle, Manager};

pub fn setup_app(app_handle: AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    use std::sync::mpsc::channel;
    use sea_orm::DbConn;
    use tracing::info;

    use crate::app::init::init_app;

    let (tx, rx) = channel::<Result<DbConn, String>>();
    let app_handle_clone = app_handle.clone();

    tauri::async_runtime::spawn(async move {
        let result = init_app(&app_handle_clone).await;
        let _ = tx.send(result.map_err(|e| e.to_string()));
    });

    let db = match rx.recv() {
        Ok(Ok(db)) => {
            info!("应用初始化成功");
            db
        }
        Ok(Err(e)) => {
            eprintln!("初始化应用失败: {}", e);
            return Err(format!("初始化失败: {}", e).into());
        }
        Err(e) => {
            eprintln!("等待初始化完成时出错: {}", e);
            return Err(format!("等待初始化失败: {}", e).into());
        }
    };

    app_handle.manage(db);

    #[cfg(debug_assertions)]
    {
        use tauri::Manager;
        if let Some(window) = app_handle.get_webview_window("main") {
            window.open_devtools();
        }
    }

    Ok(())
}
