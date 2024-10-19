use serde::{de, Serialize};
use sigscan::Signature;
use tauri::{window, Emitter, Manager};
use std::{borrow::Borrow, process::exit, sync::{Arc, Mutex}, thread};
mod memlib;
mod sigscan;

#[derive(Debug, Clone, Copy)]
struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Debug, Clone, Copy, Serialize)]
struct Vector2 {
    x: f32,
    y: f32,
}

#[derive(Debug, Clone, Copy)]
struct Vector4 {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}


struct Addresses {
    player_count: usize,
    local_player: usize,
    entity_list: usize,
    view_matrix: usize
}


#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct AcAnimateState {
    anim: i32,       // 0x0000
    frame: i32,      // 0x0004
    range: i32,      // 0x0008
    base_time: i32,  // 0x000C
    speed: f32,      // 0x0010
} // Size: 0x0014

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct AcPositionHistory {
    next_update: i32,          // 0x0000
    current_pos: i32,          // 0x0004
    num_pos: i32,              // 0x0008
    positions: [Vector3; 7],   // 0x000C
} // Size: 0x0060


#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct AcEntity {
    vtable: i32,
    origin: Vector3,
    velocity: Vector3,
    delta_position: Vector3,
    new_position: Vector3,
    angle: Vector3,
    pitch_velocity: f32,
    max_speed: f32,
    time_in_air: i32,
    radius: f32,
    eye_height: f32,
    max_eye_height: f32,
    above_eye: f32,
    in_water: bool,
    on_floor: bool,
    on_ladder: bool,
    jump_next: bool,
    jump_done: bool,
    is_crouching: bool,
    crouched_in_air: bool,
    try_crouch: bool,
    can_collide: bool,
    stuck: bool,
    scoping: bool,
    last_jump_timestamp: i32,
    last_jump_height: f32,
    last_splash: i32,
    n000000e8: i8,
    move_: i8,
    strafe: i8,
    state: u8,
    type_: u8,
    eye_height_value: f32,
    last_position: i32,
    key_left: bool,
    key_right: bool,
    key_up: bool,
    key_down: bool,
    prev_animation: [AcAnimateState; 2],
    current_animation: [AcAnimateState; 2],
    last_animation_switch_time: [i32; 2],
    last_model: [i32; 2],
    last_rendered: i32,
    health: i32,
    armour: i32,
    primary: i32,
    next_primary: i32,
    akimbo: i32,
    ammo_of_guns: [i32; 9],
    magazine_of_guns: [i32; 9],
    waittime_of_guns: [i32; 9],
    shots_of_guns: [i32; 9],
    damage_of_guns: [i32; 9],
    pad_01b0: [u8; 40],
    frags: i32,
    flag_score: i32,
    deaths: i32,
    teamkills: i32,
    last_action: i32,
    last_move: i32,
    last_pain: i32,
    last_voice_com: i32,
    last_death: i32,
    client_role: i32,
    attacking: bool,
    name: [u8; 260],
    team: i32,
    weapon_changing: i32,
    switch_weapon_to: i32,
    spectate_mode: i32,
    follow_player_cam: i32,
    ear_damage_millis: i32,
    max_roll: f32,
    max_roll_effect: f32,
    mov_roll: f32,
    eff_roll: f32,
    ffov: i32,
    scope_fov: i32,
    weapons: [i32; 9],
    prev_weapon: i32,
    weapon: i32,
    next_weapon: i32,
    primary_weapon: i32,
    next_primary_weapon: i32,
    last_attack_weapon: i32,
    history_position: AcPositionHistory,
    skin_no_team: u32,
    skin_cla: u32,
    skin_rvsf: u32,
    delta_yaw: f32,
    delta_pitch: f32,
    new_yaw: f32,
    new_pitch: f32,
    smooth_millis: i32,
    head: Vector3,
    ignored: bool,
    muted: bool,
    no_corpse: bool,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct AcEntityList {
    entities: [u32; 32], // 0x0000
} // Size: 0x0080

#[derive(Clone, Serialize)]
struct Entity {
  name: String,
  health: i32,
  screen_pos: Vector2
}

fn world_to_screen(
  position: Vector3,
  screen: &mut Vector2,
  view_matrix: [f32; 16],
  window_width: i32,
  window_height: i32,
) -> bool {

  let clip_coords = Vector4 {
    x: position.x * view_matrix[0]
      + position.y * view_matrix[4]
      + position.z * view_matrix[8]
      + view_matrix[12],
    y: position.x * view_matrix[1]
      + position.y * view_matrix[5]
      + position.z * view_matrix[9]
      + view_matrix[13],
    z: position.x * view_matrix[2]
      + position.y * view_matrix[6]
      + position.z * view_matrix[10]
      + view_matrix[14],
    w: position.x * view_matrix[3]
      + position.y * view_matrix[7]
      + position.z * view_matrix[11]
      + view_matrix[15],
  };

  if clip_coords.w < 0.1 {
    return false;
  }

  let normalized_device_coordinates = Vector3 {
    x: clip_coords.x / clip_coords.w,
    y: clip_coords.y / clip_coords.w,
    z: clip_coords.z / clip_coords.w,
  };

  screen.x = ((window_width / 2) as f32 * normalized_device_coordinates.x)
    + (normalized_device_coordinates.x + (window_width / 2) as f32);
  screen.y = -((window_height / 2) as f32 * normalized_device_coordinates.y)
    + (normalized_device_coordinates.y + (window_height / 2) as f32);

  true
}


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
            Err(_) => break
        };
        
        let player_count = lProc.read::<u32>(lAddr.player_count).unwrap_or(0);
       

        app.emit("update_player_count", player_count).unwrap();

        let local_player_ptr = lProc.read::<u32>(lAddr.local_player);    
        let entity = lProc.read::<AcEntity>(local_player_ptr.unwrap() as usize);

        let entity_list_ptr = lProc.read::<u32>(lAddr.entity_list).unwrap_or(0);
        let entity_list = lProc.read::<AcEntityList>(entity_list_ptr as usize).unwrap_or(AcEntityList { entities: [ 0x0; 32] });
        let view_matrix = lProc.read::<[f32; 16]>(lAddr.view_matrix).unwrap_or([0.0; 16]);

        let mut entities = Vec::new();

        for i in 0..32 {            
                    
            let entity = lProc.read::<AcEntity>(entity_list.entities[i] as usize);

            if(entity.is_none() || entity.unwrap().health <= 0) {
                continue;
            }

            let player = entity.unwrap();

            let nick = player.name.iter().take_while(|&&c| c != 0).map(|&c| c as char).collect::<String>();                    



            let mut screen = Vector2 { x: 0.0, y: 0.0 };


            
            if !world_to_screen(
                player.origin,
                &mut screen,
                view_matrix,
                3440,
                1440,
            ) {
            continue;
            }
            println!("{:?}",screen);
            
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

#[tauri::command(async)]
fn update_round_size(app: tauri::AppHandle, round_size: i32) {    
    app.emit_to("overlay", "change-roundsize", round_size).unwrap();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {


    let process = Arc::new(Mutex::new(memlib::from_name("ac_client.exe").unwrap()));

    let player_count_sig = Signature {
        name: "Player Count".to_string(),
        module: "ac_client.exe".to_string(),
        pattern: "8B 0D ? ? ? ? 46 3B ? 7C ? 8B 35".to_string(),
        offsets: vec![0x2],
        rip_relative: false,
        rip_offset: 0,
        extra: 0,
        relative: false
    };

    let local_player_sig = Signature {
        name: "Local Player".to_string(),
        module: "ac_client.exe".to_string(),
        pattern: "8B 0D ? ? ? ? 56 57 8B 3D".to_string(),
        offsets: vec![0x2],
        rip_relative: false,
        rip_offset: 0,
        extra: 0,
        relative: false
    };

    let entity_list_sig = Signature {
        name: "Entity List".to_string(),
        module: "ac_client.exe".to_string(),
        pattern: "A1 ? ? ? ? ? ? ? ? F6 0F 84 5F".to_string(),
        offsets: vec![0x1],
        rip_relative: false,
        rip_offset: 0,
        extra: 0,
        relative: false
    };

        let view_matrix_sig = Signature {
        name: "View Martix".to_string(),
        module: "ac_client.exe".to_string(),
        pattern: "F3 0F ? ? ? ? ? ? F3 0F ? ? 0F 28 ? 0F C6 C3 ? F3 0F ? ? ? ? ? ? F3 0F ? ? F3 0F ? ? F2 0F ? ? ? ? ? ? 0F 28 ? 0F 54 ? ? ? ? ? 0F 5A ? 66 0F ? ? 77 ? F3 0F".to_string(),
        offsets: vec![0x4],
        rip_relative: false,
        rip_offset: 0,
        extra: 0,
        relative: false
    };


    let addr = Arc::new(Mutex::new(Addresses {
        player_count: 0,
        local_player: 0,
        entity_list: 0,
        view_matrix: 0
    }));


    addr.lock().unwrap().player_count = sigscan::find_signature(&player_count_sig, &process.lock().unwrap()).unwrap_or(0);            
    addr.lock().unwrap().local_player = sigscan::find_signature(&local_player_sig, &process.lock().unwrap()).unwrap_or(0);            
    addr.lock().unwrap().entity_list = sigscan::find_signature(&entity_list_sig, &process.lock().unwrap()).unwrap_or(0);
    addr.lock().unwrap().view_matrix = sigscan::find_signature(&view_matrix_sig, &process.lock().unwrap()).unwrap_or(0);

    println!("Player count address: {:#X}", addr.lock().unwrap().player_count);
    println!("Local player address: {:#X}", addr.lock().unwrap().local_player);
    println!("Entity list address: {:#X}", addr.lock().unwrap().entity_list);
    println!("View matrix address: {:#X}", addr.lock().unwrap().view_matrix);

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
