mod godmode;
pub use godmode::GodMode;

mod infiniteammo;
use crate::utils::{Vec3, ViewMatrix};
use crate::{InternalMemory, ESP};
pub use infiniteammo::InfiniteAmmo;

use crate::offsets;
use crate::process;

const MAX_PLAYERS: usize = 32;

const TEAM_OFF: usize = 0x344;

/// alive, dead, etc.
const PLAYER_STATE_OFF: usize = 0x86;

const PLAYER_POS_OFF: usize = 0x8;

const PLAYER_NEWPOS_OFF: usize = 0x38;
const PLAYER_EYE_HEIGHT_OFF: usize = 0x60;

const PLAYER_ATTACKING_OFF: usize = 0x23c;

const GAME_MODE_OFF: usize = 0x128294;

// used for aimbot/ESP
const ALIVE_STATE: u8 = 0;

enum GameModes {
    GmodeBotTeamdeathMatch = 7,
    GmodeBotDeathMatch = 8,
    GmodeBotOneShotOneKill = 12,
    GmodeBotPistolFrenzy = 18,
    GmodeBotlss = 19,
    GmodeBotSurvivor = 20,
    GmodeBotTeamOneShotOneKill = 21,
}

impl GameModes {
    fn from_i32(int: i32) -> GameModes {
        match int {
            7 => GameModes::GmodeBotTeamdeathMatch,
            8 => GameModes::GmodeBotDeathMatch,
            12 => GameModes::GmodeBotOneShotOneKill,
            18 => GameModes::GmodeBotPistolFrenzy,
            19 => GameModes::GmodeBotlss,
            20 => GameModes::GmodeBotSurvivor,
            21 => GameModes::GmodeBotTeamOneShotOneKill,
            _ => panic!("Unsupported Game mode"),
        }
    }
}

pub struct Player {
    pub base: usize,
}

// AssaultCube custom vector type that holds pointers to enemies
// which are also of type Player
#[repr(C)]
#[derive(Clone, Copy)]
struct AcVector {
    // pointer to player pointers buffer
    player_addresses: usize,
    capacity: i32,
    elements: i32,
}

impl Player {
    pub fn get_local_player() -> Self {
        let ac_base = process::target::resolve_base_address().unwrap();
        let local_player_pointer = ac_base + offsets::LOCAL_PLAYER;

        println!("local player pointer: {:x}", local_player_pointer);

        let local_player_base: u64 = InternalMemory::read(local_player_pointer);

        println!("game base: {:x}", ac_base);
        println!("local player base: {:x}", local_player_base);

        Player {
            base: local_player_base as usize,
        }
    }

    pub fn get_players_vector() -> Vec<Self> {
        let players_base = process::target::resolve_base_address().unwrap() + offsets::PLAYERS_LIST;

        let players_ac_vector = unsafe {
            let vec_ptr = players_base as *const AcVector;
            *vec_ptr
        };

        let mut players = Vec::with_capacity(MAX_PLAYERS);

        // fill the vector of enemies
        for i in 0..players_ac_vector.elements {
            let player_address: u64 =
                InternalMemory::read(players_ac_vector.player_addresses + (i * 8) as usize);

            if player_address == 0x0 {
                continue;
            }

            players.push(Player {
                base: player_address as usize,
            });
        }

        players
    }

    pub fn get_head_position(&self) -> Vec3 {
        let mut head: [f32; 3] = [0.0; 3];
        for i in 0..3 {
            head[i] = InternalMemory::read(self.base + PLAYER_POS_OFF + i * 4);
        }
        Vec3::from(head)
    }

    fn get_eye_height(&self) -> f32 {
        InternalMemory::read(self.base + PLAYER_EYE_HEIGHT_OFF)
    }

    fn get_game_mode() -> GameModes {
        GameModes::from_i32(InternalMemory::read(
            process::target::resolve_base_address().unwrap() + GAME_MODE_OFF,
        ))
    }

    fn is_free_for_all() -> bool {
        let gamemode = Self::get_game_mode();
        match gamemode {
            GameModes::GmodeBotDeathMatch => true,
            GameModes::GmodeBotOneShotOneKill => true,
            GameModes::GmodeBotSurvivor => true,
            GameModes::GmodeBotlss => true,
            _ => false,
        }
    }

    pub fn are_enemies(&self, other: &Player) -> bool {
        if Self::is_free_for_all() {
            return true;
        }

        self.get_team() != other.get_team()
    }

    /// returns the position a player will be located at in the next frame.
    /// This is needed for a reliable aimbot
    pub fn get_new_pos(&self) -> Vec3 {
        let mut foot: [f32; 3] = [0.0; 3];
        for i in 0..3 {
            foot[i] = InternalMemory::read(self.base + PLAYER_NEWPOS_OFF + i * 4);
        }
        let mut vec = Vec3::from(foot);
        vec.z += self.get_eye_height();
        vec
    }

    /// Calculates the distance between to players in a 3D space
    pub fn distance_to(&self, other: &Player) -> f32 {
        let self_pos = self.get_head_position();
        let other_pos = other.get_head_position();

        Vec3::distance(self_pos, other_pos)
    }

    /// retuns the team the player is in
    fn get_team(&self) -> i32 {
        InternalMemory::read(self.base + TEAM_OFF)
    }

    /// returns true if the player is alive
    pub fn is_alive(&self) -> bool {
        InternalMemory::read::<u8>(self.base + PLAYER_STATE_OFF) == ALIVE_STATE
    }

    /// returns true if a player is infront of the player on the 2D screen
    pub fn is_in_view(&self) -> bool {
        let pos = self.get_head_position();
        let (window_width, window_height) = ESP::window_dimensions();
        ViewMatrix::new()
            .world_to_screen(pos, window_width, window_height)
            .0
    }

    /// triggers the ->attacking state of the player to start shooting
    pub fn shoot(&mut self) {
        InternalMemory::write(self.base + PLAYER_ATTACKING_OFF, 1 as u8)
    }

    /// stops shooting after having started through autoshoot
    pub fn stop_shoot(&mut self) {
        InternalMemory::write(self.base + PLAYER_ATTACKING_OFF, 0 as u8)
    }
}
