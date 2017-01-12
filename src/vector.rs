
use std::ops::Sub;

#[derive(Debug, PartialEq)]
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

impl<'a> Sub<&'a Vec3f> for Vec3f {
    type Output = Vec3f;

    fn sub(self, other: &Vec3f) -> Vec3f {
        // TODO really implement vector sub
        return Vec3f::new_i32(0,0,0);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sub() {
        // TODO real test
        assert_eq!(Vec3f::new_i32(1,0,0), Vec3f::new_i32(1,1,1) - &Vec3f::new_i32(2,2,2));
    }
}
