use super::vectors::Vector3;



#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AcAnimateState {
    pub anim: i32,       // 0x0000
    pub frame: i32,      // 0x0004
    pub range: i32,      // 0x0008
    pub base_time: i32,  // 0x000C
    pub speed: f32,      // 0x0010
} // Size: 0x0014

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AcPositionHistory {
    pub next_update: i32,          // 0x0000
    pub current_pos: i32,          // 0x0004
    pub num_pos: i32,              // 0x0008
    pub positions: [Vector3; 7],   // 0x000C
} // Size: 0x0060


#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AcEntity {
    pub vtable: i32,
    pub origin: Vector3,
    pub velocity: Vector3,
    pub delta_position: Vector3,
    pub new_position: Vector3,
    pub angle: Vector3,
    pub pitch_velocity: f32,
    pub max_speed: f32,
    pub time_in_air: i32,
    pub radius: f32,
    pub eye_height: f32,
    pub max_eye_height: f32,
    pub above_eye: f32,
    pub in_water: bool,
    pub on_floor: bool,
    pub on_ladder: bool,
    pub jump_next: bool,
    pub jump_done: bool,
    pub is_crouching: bool,
    pub crouched_in_air: bool,
    pub try_crouch: bool,
    pub can_collide: bool,
    pub stuck: bool,
    pub scoping: bool,
    pub last_jump_timestamp: i32,
    pub last_jump_height: f32,
    pub last_splash: i32,
    pub n000000e8: i8,
    pub move_: i8,
    pub strafe: i8,
    pub state: u8,
    pub type_: u8,
    pub eye_height_value: f32,
    pub last_position: i32,
    pub key_left: bool,
    pub key_right: bool,
    pub key_up: bool,
    pub key_down: bool,
    pub prev_animation: [AcAnimateState; 2],
    pub current_animation: [AcAnimateState; 2],
    pub last_animation_switch_time: [i32; 2],
    pub last_model: [i32; 2],
    pub last_rendered: i32,
    pub health: i32,
    pub armour: i32,
    pub primary: i32,
    pub next_primary: i32,
    pub akimbo: i32,
    pub ammo_of_guns: [i32; 9],
    pub magazine_of_guns: [i32; 9],
    pub waittime_of_guns: [i32; 9],
    pub shots_of_guns: [i32; 9],
    pub damage_of_guns: [i32; 9],
    pub pad_01b0: [u8; 40],
    pub frags: i32,
    pub flag_score: i32,
    pub deaths: i32,
    pub teamkills: i32,
    pub last_action: i32,
    pub last_move: i32,
    pub last_pain: i32,
    pub last_voice_com: i32,
    pub last_death: i32,
    pub client_role: i32,
    pub attacking: bool,
    pub name: [u8; 260],
    pub team: i32,
    pub weapon_changing: i32,
    pub switch_weapon_to: i32,
    pub spectate_mode: i32,
    pub follow_player_cam: i32,
    pub ear_damage_millis: i32,
    pub max_roll: f32,
    pub max_roll_effect: f32,
    pub mov_roll: f32,
    pub eff_roll: f32,
    pub ffov: i32,
    pub scope_fov: i32,
    pub weapons: [i32; 9],
    pub prev_weapon: i32,
    pub weapon: i32,
    pub next_weapon: i32,
    pub primary_weapon: i32,
    pub next_primary_weapon: i32,
    pub last_attack_weapon: i32,
    pub history_position: AcPositionHistory,
    pub skin_no_team: u32,
    pub skin_cla: u32,
    pub skin_rvsf: u32,
    pub delta_yaw: f32,
    pub delta_pitch: f32,
    pub new_yaw: f32,
    pub new_pitch: f32,
    pub smooth_millis: i32,
    pub head: Vector3,
    pub ignored: bool,
    pub muted: bool,
    pub no_corpse: bool,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AcEntityList {
    pub entities: [u32; 32], // 0x0000
} // Size: 0x0080