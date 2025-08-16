// hardcoded pointer addresses since they are static
pub const ENTITY_LIST_POINTER : usize =  0x591FCC;
pub const VIEW_MATRIX_POINTER : usize = 0x57DFD8;

pub mod module_base {
    pub const LOCAL_PLAYER_POINTER_OFFSET : usize = 0x17E0A8;
}

pub mod entity {
    pub const HEALTH_OFFSET : usize = 0xEC; // int 

    // Vector3
    pub const POS_X_OFFSET : usize = 0x4; // float
    pub const POS_Y_OFFSET : usize = 0x8; // float
    pub const POS_Z_OFFSET_HEAD : usize = 0xC; // float

    // Vector2
    pub const YAW_OFFSET : usize = 0x34; // float
    pub const PITCH_OFFSET : usize = 0x38; // float
}