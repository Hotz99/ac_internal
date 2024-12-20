mod ac_vector;
mod game_modes;

use crate::{offsets, player, process, utils};

pub struct Game {
    pub base_addr: usize,
}

impl Game {
    pub fn default() -> Self {
        Game {
            base_addr: utils::game_base::resolve_addr(),
        }
    }

    pub fn get_name(player_addr: usize) -> String {
        let mut name = String::new();
        // names are max 16 bytes in this game
        for i in 0..16 {
            let c = process::InternalMemory::read::<u8>(player_addr + offsets::PLAYER_NAME + i);
            if c == 0 {
                break;
            }
            name.push(c as char);
        }
        name
    }

    pub fn get_game_mode(&self) -> game_modes::GameModes {
        game_modes::GameModes::from_i32(process::InternalMemory::read(
            self.base_addr + offsets::GAME_MODE,
        ))
    }

    pub fn is_free_for_all(&self) -> bool {
        match Self::get_game_mode(self) {
            game_modes::GameModes::BotDeathMatch => true,
            game_modes::GameModes::BotOneShotOneKill => true,
            game_modes::GameModes::BotSurvivor => true,
            game_modes::GameModes::Botlss => true,
            _ => false,
        }
    }

    pub fn shoot(player_addr: usize) {
        process::InternalMemory::write(player_addr + offsets::PLAYER_IS_ATTACKING, 1u8)
    }

    pub fn stop_shoot(player_addr: usize) {
        process::InternalMemory::write(player_addr + offsets::PLAYER_IS_ATTACKING, 0 as u8)
    }

    pub fn get_players_vector(&self) -> Vec<player::Player> {
        let players_ac_vector = unsafe {
            let pointer = (self.base_addr + offsets::PLAYERS_LIST) as *const ac_vector::AcVector;
            *pointer
        };

        let mut players = Vec::with_capacity(offsets::MAX_PLAYERS);

        // fill the vector of enemies
        for i in 0..players_ac_vector.elements {
            let player_address = process::InternalMemory::read::<usize>(
                players_ac_vector.player_addrs
                    + (i * (std::mem::size_of::<usize>() as u32)) as usize,
            ) as usize;

            if player_address == 0x0 {
                continue;
            }

            players.push(player::Player::new(player_address));
        }

        players
    }
}
