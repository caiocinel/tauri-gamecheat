use crate::sigscan::Signature;



pub static PLAYER_COUNT_SIG: Signature = Signature {
    module: "ac_client.exe",
    pattern: "8B 0D ? ? ? ? 46 3B ? 7C ? 8B 35",
    offset: 0x2,
    rip_relative: false,
    rip_offset: 0,
    extra: 0,
    relative: false
};

pub static LOCAL_PLAYER_SIG : Signature = Signature {    
    module: "ac_client.exe",
    pattern: "8B 0D ? ? ? ? 56 57 8B 3D",
    offset: 0x2,
    rip_relative: false,
    rip_offset: 0,
    extra: 0,
    relative: false
};

pub static ENTITY_LIST_SIG: Signature = Signature {
    module: "ac_client.exe",
    pattern: "A1 ? ? ? ? ? ? ? ? F6 0F 84 5F",
    offset: 0x1,
    rip_relative: false,
    rip_offset: 0,
    extra: 0,
    relative: false
};

pub static VIEW_MATRIX_SIG: Signature = Signature {
    module: "ac_client.exe",
    pattern: "F3 0F ? ? ? ? ? ? F3 0F ? ? 0F 28 ? 0F C6 C3 ? F3 0F ? ? ? ? ? ? F3 0F ? ? F3 0F ? ? F2 0F ? ? ? ? ? ? 0F 28 ? 0F 54 ? ? ? ? ? 0F 5A ? 66 0F ? ? 77 ? F3 0F",
    offset: 0x4,
    rip_relative: false,
    rip_offset: 0,
    extra: 0,
    relative: false
};