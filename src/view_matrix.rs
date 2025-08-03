use crate::vector::{vec2_t, vec3_t};
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

#[allow(non_camel_case_types)]
pub struct view_matrix_t {
    pub m: [[f32; 4]; 4],
}

impl view_matrix_t {
    pub fn new() -> Self {
        view_matrix_t {
            m: [[0.0; 4]; 4],
        }
    }

    pub fn world_to_screen(
        &self,
         world : &vec3_t, 
         screen : &mut vec2_t
    ) -> bool {
        let w = self.m[3][0] * world.x + self.m[3][1] * world.y + self.m[3][2] * world.z + self.m[3][3];

        if w < 0.001 {
            return false; // behind the camera
        }

        screen.x = self.m[0][0] * world.x + self.m[0][1] * world.y + self.m[0][2] * world.z + self.m[0][3];
        screen.y = self.m[1][0] * world.x + self.m[1][1] * world.y + self.m[1][2] * world.z + self.m[1][3];

        screen.x /= w;
        screen.y /= w;

        screen.x = (*SCREEN_WIDTH / 2.0) + (screen.x * *SCREEN_WIDTH) / 2.0;
        screen.y = (*SCREEN_HEIGHT / 2.0) - (screen.y * *SCREEN_HEIGHT) / 2.0;

        true
    }

}