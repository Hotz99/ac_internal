use crate::{offsets, process, utils};

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(C)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn from(position: [f32; 3]) -> Self {
        Self {
            x: position[0],
            y: position[1],
            z: position[2],
        }
    }

    pub fn eucledian_distance(a: Vec3, b: Vec3) -> f32 {
        let vector = a - b;

        f32::sqrt(vector.x.powi(2) + vector.y.powi(2) + vector.z.powi(2))
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({:.2}, {:.2}, {:.2})", self.x, self.y, self.z)
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

// http://www.opengl-tutorial.org/beginners-tutorials/tutorial-3-matrices/#the-view-matrix
// https://solarianprogrammer.com/2013/05/22/opengl-101-matrices-projection-view-model/
pub struct ViewMatrix {
    base_addr: usize,
}

impl ViewMatrix {
    pub fn default() -> Self {
        let view_matrix_addr = utils::game_base::resolve_addr() + offsets::VIEW_MATRIX;

        ViewMatrix {
            base_addr: view_matrix_addr,
        }
    }

    fn read_view_matrix(&self) -> [[f32; 4]; 4] {
        let mut matrix = [[0.0f32, 0.0, 0.0, 0.0]; 4];
        let mut row = 0;
        let mut col = 0;
        for i in 0usize..16 {
            if col == 4 {
                col = 0;
                row += 1;
            }

            matrix[row][col] = process::InternalMemory::read(self.base_addr + i * 4);
            col += 1;
        }

        matrix
    }

    pub fn world_to_screen(
        &self,
        pos: Vec3,
        screen_width: i32,
        screen_height: i32,
    ) -> (bool, f32, f32) {
        let matrix = self.read_view_matrix();

        let screen_x =
            (pos.x * matrix[0][0]) + (pos.y * matrix[1][0]) + (pos.z * matrix[2][0]) + matrix[3][0];
        let screen_y =
            (pos.x * matrix[0][1]) + (pos.y * matrix[1][1]) + (pos.z * matrix[2][1]) + matrix[3][1];
        let screen_z =
            (pos.x * matrix[0][2]) + (pos.y * matrix[1][2]) + (pos.z * matrix[2][2]) + matrix[3][2];
        let screen_w =
            (pos.x * matrix[0][3]) + (pos.y * matrix[1][3]) + (pos.z * matrix[2][3]) + matrix[3][3];

        // if entity is behind the camera
        if screen_w < 0.1 {
            return (false, 0.0, 0.0);
        }

        // normalized device coordinates
        let mut ndc = Vec3::from([screen_x, screen_y, screen_z]);
        ndc.x = screen_x / screen_w;
        ndc.y = screen_y / screen_w;
        ndc.z = screen_z / screen_w;

        let x = (screen_width as f32 / 2.0 * ndc.x) + (ndc.x + screen_width as f32 / 2.0);
        let y = -(screen_height as f32 / 2.0 * ndc.y) + (ndc.y + screen_height as f32 / 2.0);

        (true, x, y)
    }
}
