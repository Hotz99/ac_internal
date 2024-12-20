use crate::{game, hacks, offsets, player, process};
use std::rc::Rc;

#[allow(dead_code)]
pub struct Instance {
    pub game: Rc<game::Game>,
    pub local_player: Rc<player::Player>,
    pub god_mode: hacks::godmode::GodMode,
    pub infinite_ammo: hacks::ammo::InfiniteAmmo,
    pub esp: hacks::esp::ESP,
}

impl Instance {
    pub fn default() -> Self {
        let game = Rc::new(game::Game::default());

        let local_player = Rc::new(player::Player::new(process::InternalMemory::read::<u64>(
            game.base_addr + offsets::LOCAL_PLAYER,
        ) as usize));

        let esp = hacks::esp::ESP::new(Rc::clone(&game), Rc::clone(&local_player));
        let god_mode = hacks::godmode::GodMode::new(game.base_addr, local_player.base_addr);
        let infinite_ammo = hacks::ammo::InfiniteAmmo::default();

        Instance {
            game,
            local_player,
            esp,
            god_mode,
            infinite_ammo,
        }
    }
}
