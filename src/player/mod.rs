use crate::{hacks, offsets, process, utils};

// used for aimbot/ESP
const ALIVE_STATE: u8 = 0;

pub struct Player {
    pub base_addr: usize,
}

impl Player {
    pub fn new(base_addr: usize) -> Self {
        Player { base_addr }
    }

    pub fn get_name(&self) -> String {
        let mut name = String::new();
        // names are max 16 bytes in this game
        for i in 0..16 {
            let c = process::InternalMemory::read::<u8>(self.base_addr + offsets::PLAYER_NAME + i);
            if c == 0 {
                break;
            }
            name.push(c as char);
        }
        name
    }

    pub fn is_alive(&self) -> bool {
        process::InternalMemory::read::<u8>(self.base_addr + offsets::PLAYER_STATE) == ALIVE_STATE
    }

    pub fn is_enemy(&self, other: &Player) -> bool {
        // TODO
        //if game::Game::is_free_for_all() {
        //    return true;
        //}

        self.get_team() != other.get_team()
    }

    pub fn get_head_pos(&self) -> utils::math::Vec3 {
        let mut head: [f32; 3] = [0.0; 3];
        for i in 0..3 {
            head[i] = process::InternalMemory::read(self.base_addr + offsets::PLAYER_POS + i * 4);
        }
        utils::math::Vec3::from(head)
    }

    pub fn get_eye_height(&self) -> f32 {
        process::InternalMemory::read(self.base_addr + offsets::PLAYER_EYE_HEIGHT)
    }

    pub fn is_in_view(player: &Self) -> bool {
        let head_pos = player.get_head_pos();
        let (window_width, window_height) = hacks::esp::ESP::get_window_dimensions();
        utils::math::ViewMatrix::default()
            .world_to_screen(head_pos, window_width, window_height)
            .0
    }

    // returns next frame player position
    // needed for aimbot
    pub fn get_next_frame_pos(&self) -> utils::math::Vec3 {
        let mut foot: [f32; 3] = [0.0; 3];
        for i in 0..3 {
            foot[i] =
                process::InternalMemory::read(self.base_addr + offsets::PLAYER_NEWPOS + i * 4);
        }
        let mut vec = utils::math::Vec3::from(foot);
        vec.z += self.get_eye_height();
        vec
    }

    fn get_team(&self) -> i32 {
        process::InternalMemory::read(self.base_addr + offsets::TEAM)
    }
}
