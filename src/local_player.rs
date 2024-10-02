#[repr(C)]
pub struct Point3D {
    x: f32,
    y: f32,
    z: f32,
}

#[repr(C)]
pub struct LocalPlayer {
    pub unknown1: [u8; 0x4],              // +0x8
    pub head_position: Point3D,           // +0xC
    pub unknown2: [u8; 0x24],             // +0x18
    pub position: Point3D,                // +0x3C
    pub view: Point3D,                    // +0x48
    pub unknown3: [u8; 0x8],              // +0x60
    pub jump_fall_speed: i32,             // +0x68
    pub no_clip: f32,                     // +0x6C
    pub unknown4: [u8; 0x14],             // +0x70
    pub is_immobile: i32,                 // +0x84
    pub unknown5: [u8; 0xE],              // +0x88
    pub state: i8,                        // +0x96
    pub unknown6: [u8; 0x75],             // +0x97
    pub hp: i32,                          // +0x10C
    pub armor: i32,                       // +0x110
    pub unknown7: [u8; 0xC],              // +0x114
    pub dual_pistol_enabled: i8,          // +0x120
    pub unknown8: [u8; 0x7],              // +0x121
    pub pistol_reserve_ammos: i32,        // +0x128
    pub carabine_reserve_ammos: i32,      // +0x12C
    pub shotgun_reserve_ammos: i32,       // +0x130
    pub smg_reserve_ammos: i32,           // +0x134
    pub sniper_rifle_reserve_ammos: i32,  // +0x138
    pub assault_rifle_reserve_ammos: i32, // +0x13C
    pub unknown9: [u8; 0x8],              // +0x140
    pub double_pistol_reserve_ammos: i32, // +0x148
    pub unknown10: [u8; 0x4],             // +0x14C
    pub pistol_loaded_ammos: i32,         // +0x150
    pub carabine_loaded_ammos: i32,       // +0x154
    pub shotgun_loaded_ammos: i32,        // +0x158
    pub smg_loaded_ammos: i32,            // +0x15C
    pub sniper_rifle_loaded_ammos: i32,   // +0x160
    pub assault_rifle_loaded_ammos: i32,  // +0x164
    pub unknown11: [u8; 0x4],             // +0x168
    pub grenades: i32,                    // +0x16C
    pub double_pistol_loaded_ammos: i32,  // +0x170
    pub knife_slash_delay: i32,           // +0x174
    pub pistol_shoot_delay: i32,          // +0x178
    pub carabine_shoot_delay: i32,        // +0x17C
    pub shotgun_shoot_delay: i32,         // +0x180
    pub smg_shoot_delay: i32,             // +0x184
    pub sniper_rifle_shoot_delay: i32,    // +0x188
    pub assault_rifle_shoot_delay: i32,   // +0x18C
    pub unknown12: [u8; 0x8],             // +0x190
    pub double_pistol_shoot_delay: i32,   // +0x198
    pub unknown13: [u8; 0x7C],            // +0x19C
    pub number_of_deaths: i32,            // +0x218
    pub unknown14: [u8; 0x1D],            // +0x21C
    pub nickname: [u8; 16],               // +0x239
    pub unknown15: [u8; 0xF7],            // +0x249
    pub team: i8,                         // +0x340
}
