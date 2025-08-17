use crate::vector::{Vec2, Vec3};
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};
use memflow::prelude::Pod;


#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, Pod)]
pub struct ViewMatrix {
    pub m: [[f32; 4]; 4],
}

impl ViewMatrix {
    pub fn new() -> Self {
        ViewMatrix {
            m: [[0.0; 4]; 4],
        }
    }

    pub fn world_to_screen(
        &self,
         ent_bone_pos : &Vec3, 
         screen : &mut Vec2
    ) -> bool {
        let w = self.m[3][0] * ent_bone_pos.x + self.m[3][1] * ent_bone_pos.y + self.m[3][2] * ent_bone_pos.z + self.m[3][3];

        if w < 0.001 {
            return false; // behind the camera
        }

        screen.x = self.m[0][0] * ent_bone_pos.x + self.m[0][1] * ent_bone_pos.y + self.m[0][2] * ent_bone_pos.z + self.m[0][3];
        screen.y = self.m[1][0] * ent_bone_pos.x + self.m[1][1] * ent_bone_pos.y + self.m[1][2] * ent_bone_pos.z + self.m[1][3];

        screen.x /= w;
        screen.y /= w;

        screen.x = (*SCREEN_WIDTH / 2.0) + (screen.x * *SCREEN_WIDTH) / 2.0;
        screen.y = (*SCREEN_HEIGHT / 2.0) - (screen.y * *SCREEN_HEIGHT) / 2.0;

        true
    }

}