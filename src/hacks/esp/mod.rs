pub mod esp_box;

use crate::{game, player, utils};
use std::rc::Rc;

pub struct ESP {
    game: Rc<game::Game>,
    local_player: Rc<player::Player>,
    pub is_enabled: bool,
}

impl ESP {
    pub fn new(game: Rc<game::Game>, local_player: Rc<player::Player>) -> Self {
        ESP {
            game,
            local_player,
            is_enabled: false,
        }
    }

    pub fn toggle(&mut self) -> bool {
        self.is_enabled = !self.is_enabled;
        self.is_enabled
    }

    // set GL to orthographic (3d to 2d) projection mode
    fn set_ortho(
        &self,
    ) -> (
        utils::bindings::gl_bindings::GLint,
        utils::bindings::gl_bindings::GLint,
    ) {
        unsafe {
            // save current state
            utils::bindings::gl_bindings::glPushAttrib(
                utils::bindings::gl_bindings::GL_ALL_ATTRIB_BITS,
            );

            // save current matrix by pushing it onto stack
            utils::bindings::gl_bindings::glPushMatrix();

            let viewport = ESP::get_window_dimensions();

            utils::bindings::gl_bindings::glViewport(0, 0, viewport.0, viewport.1);

            // go into projection mode
            utils::bindings::gl_bindings::glMatrixMode(utils::bindings::gl_bindings::GL_PROJECTION);

            // load blank matrix
            utils::bindings::gl_bindings::glLoadIdentity();

            utils::bindings::gl_bindings::glOrtho(
                0.0,
                viewport.0.into(),
                viewport.1.into(),
                0.0,
                -1.0,
                1.0,
            );

            utils::bindings::gl_bindings::glMatrixMode(utils::bindings::gl_bindings::GL_MODELVIEW);
            utils::bindings::gl_bindings::glLoadIdentity();
            utils::bindings::gl_bindings::glDisable(utils::bindings::gl_bindings::GL_DEPTH_TEST);

            (viewport.0, viewport.1)
        }
    }

    // restore 3d projection mode by popping 3d matrix + attributes off stack
    fn restore(&self) {
        unsafe {
            utils::bindings::gl_bindings::glPopMatrix();
            utils::bindings::gl_bindings::glPopAttrib();
        }
    }

    pub fn draw(&self) {
        let players = self.game.get_players_vector();

        if players.is_empty() {
            return;
        }

        let window_dimensions = self.set_ortho();

        for player in players.iter() {
            if !player.is_alive() {
                continue;
            }

            esp_box::draw(&self.local_player, &player, window_dimensions);
        }

        // restore 3d projection mode after drawing 2d elements
        self.restore();
    }

    // resolve current window viewport
    pub fn get_window_dimensions() -> (i32, i32) {
        let mut viewport = [0; 4];
        unsafe {
            let ptr_viewport = &mut viewport[0] as *mut utils::bindings::gl_bindings::GLint;
            utils::bindings::gl_bindings::glGetIntegerv(
                utils::bindings::gl_bindings::GL_VIEWPORT,
                ptr_viewport,
            );
        };

        (viewport[2], viewport[3])
    }
}
