use crate::{player, utils};

// copied from https://www.youtube.com/watch?v=kGDKQXgxIrY&t=1125s
// used for scaling ESP box
const VIRTUAL_SCREEN_WIDTH: i32 = 800;
const GAME_UNIT_MAGIC: usize = 400;
const PLAYER_HEIGHT: f32 = 7.25;
const PLAYER_WIDTH: f32 = 3.5;
const PLAYER_ASPECT_RATIO: f32 = PLAYER_HEIGHT / PLAYER_WIDTH;

const LINE_WIDTH: f32 = 2.0;

fn scale(distance: f32, window_width: i32) -> f32 {
    (GAME_UNIT_MAGIC as f32 / distance) * (window_width / VIRTUAL_SCREEN_WIDTH) as f32
}

pub fn draw(
    local_player: &player::Player,
    other_player: &player::Player,
    window_dimensions: (
        utils::bindings::gl_bindings::GLint,
        utils::bindings::gl_bindings::GLint,
    ),
) {
    let (r, g, b) = if other_player.is_enemy(&local_player) {
        (255, 0, 0)
    } else {
        (0, 255, 0)
    };

    let player_screen_coords = utils::math::ViewMatrix::default().world_to_screen(
        other_player.get_head_pos(),
        window_dimensions.0,
        window_dimensions.1,
    );

    if !player_screen_coords.0 {
        return;
    }

    let x = player_screen_coords.1;
    let y = player_screen_coords.2;

    let distance = utils::math::Vec3::eucledian_distance(
        local_player.get_head_pos(),
        other_player.get_head_pos(),
    );

    let scale = scale(distance, window_dimensions.0);

    let x = x - scale;
    let y = y - scale;
    let width = scale * 2.0;
    let height = scale * PLAYER_ASPECT_RATIO * 2.0;

    // actually draw ESP box
    // our library is loaded into the game process memory, then simply execute OpenGL commands
    unsafe {
        utils::bindings::gl_bindings::glLineWidth(LINE_WIDTH);
        utils::bindings::gl_bindings::glColor3ub(r, g, b);
        utils::bindings::gl_bindings::glBegin(utils::bindings::gl_bindings::GL_LINE_STRIP);
        utils::bindings::gl_bindings::glVertex2f(x, y);
        utils::bindings::gl_bindings::glVertex2f(x + width, y);
        utils::bindings::gl_bindings::glVertex2f(x + width, y + height);
        utils::bindings::gl_bindings::glVertex2f(x, y + height);
        utils::bindings::gl_bindings::glVertex2f(x, y);
        utils::bindings::gl_bindings::glEnd();
    }
}
