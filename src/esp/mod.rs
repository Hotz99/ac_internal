mod gl_bindings;
use gl_bindings::*;

use crate::{game, player};
use std::rc::Rc;

mod esp_box;
use esp_box::ESPBox;

const ENEMY_ESP_COLOR: [GLubyte; 3] = [252, 18, 10];
const TEAM_ESP_COLOR: [GLubyte; 3] = [38, 217, 50];

pub struct ESP {
    game: Rc<game::Game>,
    local_player: Rc<player::Player>,
    esp_box: ESPBox,
}

impl ESP {
    pub fn new(game: Rc<game::Game>, local_player: Rc<player::Player>) -> Self {
        ESP {
            game,
            local_player,
            esp_box: ESPBox::new(ENEMY_ESP_COLOR, TEAM_ESP_COLOR),
        }
    }

    // switch openGL mode to 2d matrix and pushes current state onto a stack
    // so we can pop it later
    // returns current window dimensions
    fn switch_to_2d(&self) -> (GLint, GLint) {
        unsafe {
            // save current state
            gl_bindings::glPushAttrib(GL_ALL_ATTRIB_BITS);

            // save current matrix
            gl_bindings::glPushMatrix();

            // obtain and set current viewport (position and dimensions of the window)
            // for new matrix
            let mut viewport: [GLint; 4] = [0; 4];
            let viewport_ptr = &mut viewport[0] as *mut GLint;
            gl_bindings::glGetIntegerv(GL_VIEWPORT, viewport_ptr);
            gl_bindings::glViewport(0, 0, viewport[2], viewport[3]);

            // go into projection mode
            gl_bindings::glMatrixMode(GL_PROJECTION);

            // load blank matrix
            gl_bindings::glLoadIdentity();

            gl_bindings::glOrtho(0.0, viewport[2].into(), viewport[3].into(), 0.0, -1.0, 1.0);

            gl_bindings::glMatrixMode(GL_MODELVIEW);
            gl_bindings::glLoadIdentity();
            gl_bindings::glDisable(GL_DEPTH_TEST);

            (viewport[2], viewport[3])
        }
    }

    // restore 3d projection mode
    fn restore(&self) {
        unsafe {
            gl_bindings::glPopMatrix();
            gl_bindings::glPopAttrib();
        }
    }

    pub fn draw(&self) {
        let window_dimensions = self.switch_to_2d();
        let players = self.game.get_players_vector();

        for player in players.iter() {
            if !player.is_alive() {
                continue;
            }

            self.esp_box
                .draw_box(&self.local_player, &player, window_dimensions);

            println!("player name: {:?}", player.get_name());
        }

        self.restore();
    }

    // can be used by other modules to get information about the window
    pub fn window_dimensions() -> (i32, i32) {
        let mut viewport: [GLint; 4] = [0; 4];
        unsafe {
            let viewport_ptr = &mut viewport[0] as *mut GLint;
            gl_bindings::glGetIntegerv(GL_VIEWPORT, viewport_ptr);
        };
        (viewport[2], viewport[3])
    }
}
