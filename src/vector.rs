
pub struct Vec3f {
    x: f32,
    y: f32,
    z: f32
}

impl Vec3f {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3f {
        return Vec3f{x: x, y: y, z: z};
    }

    pub fn new_i32(x: i32, y: i32, z: i32) -> Vec3f {
        return Vec3f{x: (x as f32), y: (y as f32), z: (z as f32)};
    }
}
