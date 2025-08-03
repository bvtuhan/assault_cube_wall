use std::ops::Sub;

#[allow(non_camel_case_types)]
pub struct vec3_t {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl std::ops::Add for vec3_t {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        vec3_t {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::Sub for vec3_t {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        vec3_t {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl vec3_t {

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        vec3_t { x, y, z }
    }

    /// euklidean distance, a.k.a ||v||, length of the vector
    pub fn length(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn delta_to(&self, other: &Self) -> Self {
        vec3_t {
            x: other.x - self.x,
            y: other.y - self.y,
            z: other.z - self.z,
        }
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn distance_to(&self, other: &Self) -> f32 {
        self.delta_to(other).length()
    }

    pub fn calc_yaw_to(&self, other : &Self) -> f32 {
        let delta = self.delta_to(other);
        ((delta.y).atan2(delta.x)) * 180.0 / std::f32::consts::PI
    }

    pub fn calc_pitch_to(&self, other: &Self) -> f32 {
        let delta = self.delta_to(other);
        ((-delta.z) / self.distance_to(other)).asin() * 180.0 / std::f32::consts::PI
    }   
}

#[allow(non_camel_case_types)]
pub struct vec2_t {
    pub x: f32,
    pub y: f32,
}

impl vec2_t {
    pub fn new(x: f32, y: f32) -> Self {
        vec2_t { x, y }
    }

    pub fn length(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn distance_to(&self, other: &Self) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}