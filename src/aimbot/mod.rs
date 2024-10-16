mod norecoil;
use norecoil::NoRecoilSpread;

use crate::player::Player;
use crate::process::InternalMemory;
use crate::utils::Vec3;

use crate::offsets;
use crate::process;

use std::f32::consts::PI;

pub struct AimBot {
    player: Player,

    /// no recoil and no spread hook
    pub norecoil_spread: NoRecoilSpread,

    /// shoot automatically if an enemy is in the crosshair
    autoshoot: bool,

    enabled: bool,
}

#[link(name = "bot_trampoline", kind = "static")]
extern "C" {
    fn bot_isvisible(func_addr: usize, from: *const Vec3, to: *const Vec3) -> u8;
}

impl AimBot {
    pub fn default() -> AimBot {
        let player = Player::get_local_player();
        AimBot {
            autoshoot: false,
            player,
            norecoil_spread: NoRecoilSpread::new(),
            enabled: false,
        }
    }

    fn get_camera1() -> usize {
        InternalMemory::read::<u64>(
            process::target::resolve_base_address().unwrap() + offsets::CAMERA1,
        ) as usize
    }

    pub fn enable_autoshoot(&mut self) {
        self.autoshoot = true
    }

    // compute player position for next frame when it moves
    // returns camera angle to point at this enemy
    fn get_camera_angle_for_target(&self, enemy: &Player) -> (f32, f32) {
        let target_pos = enemy.get_new_pos();
        let self_pos = self.player.get_new_pos();
        let dx = target_pos.x - self_pos.x;
        let dy = target_pos.y - self_pos.y;
        let dz = target_pos.z - self_pos.z;

        // horizontal angle to player
        let yaw = dy.atan2(dx) * 180.0 / PI;

        let distance = self.player.distance_to(enemy);
        let pitch = dz.atan2(distance) * 180.0 / PI;

        (yaw + 90.0, pitch)
    }

    // calling game's IsVisible() requires c++ trampoline
    fn is_player_visible(&self, enemy: &Player) -> bool {
        let res = unsafe {
            let is_visible_addr =
                process::target::resolve_base_address().unwrap() + offsets::IS_VISIBLE;
            let from = &self.player.get_head_position() as *const Vec3;
            let to = &enemy.get_head_position() as *const Vec3;
            bot_isvisible(is_visible_addr, from, to)
        };
        res == 1
    }

    /// Called after each frame by the main SwapWindow hook. Handles findings a target
    /// to aim at and updating camera perspective
    pub fn logic(&mut self) {
        // stop shooting if we are not locked onto a target
        if self.autoshoot {
            self.player.stop_shoot();
        }

        // don't to anything if the aimbot is disabled
        if !self.enabled {
            return;
        }

        // obtain a list of all enemies which are alive
        let players: Vec<Player> = Player::get_players_vector()
            .into_iter()
            .filter(|p| p.is_alive() && self.player.are_enemies(p))
            .collect();

        // no need to do anything if no enemies are alive
        if players.len() == 0 {
            return;
        }

        // find the closest enemy that we can shoot at
        let mut best_dist = f32::INFINITY;
        let mut target = None;
        for p in players.iter() {
            let pdist = self.player.distance_to(p);
            if pdist < best_dist && self.is_player_visible(p) {
                best_dist = pdist;
                target = Some(p);
            }
        }

        // verify that a target was found to point at
        if target.is_none() {
            return;
        }

        let target = target.unwrap();

        // update the camera position to point at the enemy
        let (yaw, pitch) = self.get_camera_angle_for_target(target);

        // verify camera1 is valid!
        if Self::get_camera1() == 0x0 {
            return;
        }

        // update the camera position
        InternalMemory::write(Self::get_camera1() + offsets::YAW, yaw);
        InternalMemory::write(Self::get_camera1() + offsets::PITCH, pitch);

        // kill something
        if self.autoshoot {
            self.player.shoot();
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn toggle(&mut self) {
        if self.enabled {
            self.disable();
        } else {
            self.enable();
        }
    }
}
