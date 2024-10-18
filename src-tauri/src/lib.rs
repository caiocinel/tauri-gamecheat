use serde::de;
use sigscan::Signature;
use tauri::{Manager, Emitter};
use std::{borrow::Borrow, process::exit, sync::{Arc, Mutex}, thread};
mod memlib;
mod sigscan;

#[derive(Debug, Clone, Copy)]
struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}


struct Addresses {
    player_count: usize,
    local_player: usize
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
            Err(_) => break
        };
        
        let player_count = lProc.read::<u32>(lAddr.player_count).unwrap_or(0);
       

        app.emit("update_player_count", player_count).unwrap();

        let crounch: *mut AcEntity = std::ptr::null_mut();
        

        let result = lProc.read_ptr::<AcEntity>(crounch, lAddr.local_player, 0);
        
        
        println!("{}",result);

        unsafe {
            println!("Local player origin: {:?}", (*crounch));
        }
        
                        







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

    let addr = Arc::new(Mutex::new(Addresses {
        player_count: 0,
        local_player: 0
    }));


    addr.lock().unwrap().player_count = sigscan::find_signature(&player_count_sig, &process.lock().unwrap()).unwrap_or(0);            
    addr.lock().unwrap().local_player = sigscan::find_signature(&local_player_sig, &process.lock().unwrap()).unwrap_or(0);            

    println!("Player count address: {:#X}", addr.lock().unwrap().player_count);
    println!("Local player address: {:#X}", addr.lock().unwrap().local_player);

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
