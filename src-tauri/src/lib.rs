use sigscan::Signature;
use tauri::{Manager, Emitter};
use std::{borrow::Borrow, process::exit, sync::{Arc, Mutex}, thread};
mod memlib;
mod sigscan;


struct Addresses {
    player_count: usize
}

    // println!("Player count address: {:#X}", addresses.player_count);    
    // println!("Player count: {:?}", process.lock().unwrap().read::<u32>(addresses.player_count));



#[tauri::command]
fn render(app: tauri::AppHandle, addresses: tauri::State<'_, Arc<Mutex<Addresses>>>, process: tauri::State<'_, Arc<Mutex<memlib::Process>>>) {        
    let view =  app.get_webview_window("overlay").unwrap();        
    view.eval("window.location.reload();").unwrap();
    view.open_devtools();
    view.show().unwrap();    
}

#[tauri::command(async)]
async fn start(app: tauri::AppHandle, addresses: tauri::State<'_, Arc<Mutex<Addresses>>>, process: tauri::State<'_, Arc<Mutex<memlib::Process>>>) -> Result<(), String> {    
            
    let process = Arc::clone(&process);
    let addresses = Arc::clone(&addresses);
    

    thread::spawn(move || loop {
        
        let lProc = process.try_lock();

        if lProc.is_err() {
            break;
        }
        let mut lProc = match lProc {
            Ok(x) => x,
            Err(_) => break,
        };

        let lAddr = addresses.try_lock();

        if lAddr.is_err() {
            break;
        }

        let mut lAddr = match lAddr {
            Ok(x) => x,
            Err(_) => break,
        };


        println!("Player count Addr: {:#?}", lAddr.player_count);
        let player_count = lProc.read::<u32>(lAddr.player_count).unwrap_or(0);
        println!("Player count: {:#?}", player_count);
		
        thread::sleep(std::time::Duration::from_secs(1));
    });

    Ok(())
}

#[tauri::command(async)]
fn update_round_size(app: tauri::AppHandle, round_size: i32) {    
    app.emit_to("overlay", "change-roundsize", round_size).unwrap();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {


    let process = Arc::new(Mutex::new(memlib::from_name("ac_client.exe").unwrap()));

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

    let addr = Arc::new(Mutex::new(Addresses {
        player_count: 0
    }));


    addr.lock().unwrap().player_count = sigscan::find_signature(&sig, &process.lock().unwrap()).unwrap_or(0);            

    tauri::Builder::default()
        .manage(addr.clone())
        .manage(process.clone())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![render, update_round_size, start])
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
