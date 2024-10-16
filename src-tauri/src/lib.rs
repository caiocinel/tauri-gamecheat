use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn render(app: tauri::AppHandle) {    
    app.get_webview_window("overlay").unwrap().show().unwrap();    
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![render])
        .setup(|app| {
            let webview_url = tauri::WebviewUrl::App("/overlay".into());

            
            let window = tauri::WebviewWindowBuilder::new(app, "overlay", webview_url.clone())
                .fullscreen(true)
                .resizable(false)
                .transparent(true)
                .always_on_top(true)
                .title("Overlay")
                .skip_taskbar(true)
                .visible(false)
                .build()
                .unwrap();           

            window.set_ignore_cursor_events(true)?;            

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
