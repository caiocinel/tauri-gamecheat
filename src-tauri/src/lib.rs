use sigscan::Signature;
use tauri::{Manager, Emitter};
use std::process::exit;
mod memlib;
mod sigscan;


#[tauri::command]
fn render(app: tauri::AppHandle) {    
    let view =  app.get_webview_window("overlay").unwrap();        
    view.eval("window.location.reload();").unwrap();
    view.open_devtools();
    view.show().unwrap();    
}


#[tauri::command(async)]
fn update_round_size(app: tauri::AppHandle, round_size: i32) {
    app.emit_to("overlay", "change-roundsize", round_size).unwrap();
}

#[tauri::command]
async fn update_player_data(app: tauri::AppHandle) {

    app.emit_to("overlay", "update_player_data", 1).unwrap();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {


        let process = memlib::from_name("ac_client.exe")
        .ok_or_else(|| {
            println!("Could not open process {}!", "ac_client.exe");
            exit(1);
        })
        .unwrap();

    println!("456");


    let sig = Signature {
        name: "Base".to_string(),
        module: "ac_client.exe".to_string(),
        pattern: "8B 0D ? ? ? ? 46 3B ? 7C ? 8B 35".to_string(),
        offsets: vec![0x2],
        rip_relative: false,
        rip_offset: 0,
        extra: 0,
        relative: false
    };
    println!("789");

    match sigscan::find_signature(&sig, &process) {
        Ok(r) => {            
            println!("Found signature: {} => {:#X}", "Base", r);            

            //print the value at the address
            let value = process.read::<u32>(r).unwrap();
            println!("Value at address: {}", value);

        }
        Err(err) => println!("{} sigscan failed: {:?}", "Base", err),
    };



    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![render, update_round_size, update_player_data])
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::Destroyed => {                
                window.app_handle().exit(0);
            }
            _ => {}
        })
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
