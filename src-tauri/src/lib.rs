use cheat::{signatures::{ENTITY_LIST_SIG, LOCAL_PLAYER_SIG, PLAYER_COUNT_SIG, VIEW_MATRIX_SIG}, structs::{game_classes::{AcEntity, AcEntityList}, general::{Addresses, Entity}, vectors::Vector2}, utils::world_to_screen};
use tauri::{Emitter, Manager};
use std::{sync::{Arc, Mutex}, thread};
mod memlib;
mod sigscan;
mod cheat;


#[tauri::command]
fn render(app: tauri::AppHandle) {        
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
        
        let l_proc = process.try_lock();

        if l_proc.is_err() {
            break;
        }
        let l_proc = match l_proc {
            Ok(x) => x,
            Err(_) => break,
        };

        let l_addr = addresses.try_lock();

        if l_addr.is_err() {
            break;
        }

        let l_addr = match l_addr {
            Ok(x) => x,
            Err(_) => break
        };
        
        let player_count = l_proc.read::<u32>(l_addr.player_count).unwrap_or(0);
       

        app.emit("update_player_count", player_count).unwrap();

        let local_player_ptr = l_proc.read::<u32>(l_addr.local_player);    

        let _local_player = l_proc.read::<AcEntity>(local_player_ptr.unwrap() as usize);

        let entity_list_ptr = l_proc.read::<u32>(l_addr.entity_list).unwrap_or(0);
        let entity_list = l_proc.read::<AcEntityList>(entity_list_ptr as usize).unwrap_or(AcEntityList { entities: [ 0x0; 32] });
        let view_matrix = l_proc.read::<[f32; 16]>(l_addr.view_matrix).unwrap_or([0.0; 16]);

        let mut entities = Vec::new();

        for i in 0..32 {            
                    
            let entity = l_proc.read::<AcEntity>(entity_list.entities[i] as usize);

            if entity.is_none() || entity.unwrap().health <= 0 {
                continue;
            }

            let player = entity.unwrap();

            let nick = player.name.iter().take_while(|&&c| c != 0).map(|&c| c as char).collect::<String>();                    



            let mut screen = Vector2 { x: 0.0, y: 0.0 };


            
            if !world_to_screen(player.origin, &mut screen, view_matrix, 3440, 1440,) {
                continue;
            }            
            
            entities.push(Entity {
                name: nick,
                health: player.health,
                screen_pos: screen
            });
        }   

        app.emit("update-entitylist", entities).unwrap();
                        
        std::thread::sleep(std::time::Duration::from_millis(16)); 
    });
    
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let process = Arc::new(Mutex::new(memlib::from_name("ac_client.exe").unwrap()));


    let player_count = sigscan::find_signature(&PLAYER_COUNT_SIG, &process.lock().unwrap()).unwrap_or(0);
    let local_player = sigscan::find_signature(&LOCAL_PLAYER_SIG, &process.lock().unwrap()).unwrap_or(0);
    let entity_list = sigscan::find_signature(&ENTITY_LIST_SIG, &process.lock().unwrap()).unwrap_or(0);
    let view_matrix = sigscan::find_signature(&VIEW_MATRIX_SIG, &process.lock().unwrap()).unwrap_or(0);


    let addr = Arc::new(Mutex::new(Addresses {
        player_count,
        local_player,
        entity_list,
        view_matrix
    }));

    
    println!("Player count address: {:#X}", addr.lock().unwrap().player_count);
    println!("Local player address: {:#X}", addr.lock().unwrap().local_player);
    println!("Entity list address: {:#X}", addr.lock().unwrap().entity_list);
    println!("View matrix address: {:#X}", addr.lock().unwrap().view_matrix);

    tauri::Builder::default()
        .manage(addr.clone())
        .manage(process.clone())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![render, start])
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
