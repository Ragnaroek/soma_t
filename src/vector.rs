
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

    pub fn crossProduct(self, other: &Vec3f) -> Vec3f {
        return Vec3f::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x);
    }

    pub fn dotProduct(self, other: &Vec3f) -> f32 {
        return self.x * other.x +
               self.y * other.y +
               self.z * other.z;
    }
}

impl<'a> Sub<&'a Vec3f> for Vec3f {
    type Output = Vec3f;

    fn sub(self, other: &Vec3f) -> Vec3f {
        return Vec3f::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sub() {
        assert_eq!(Vec3f::new_i32(1,1,1), Vec3f::new_i32(2,2,2) - &Vec3f::new_i32(1,1,1));
        assert_eq!(Vec3f::new(-3.04, -4.04, -5.04), Vec3f::new(-4.14, -5.14, -6.14) - &Vec3f::new(-1.1, -1.1, -1.1));
    }

    #[test]
    fn test_crossProduct() {
        assert_eq!(Vec3f::new_i32(0,-1,0), Vec3f::new_i32(1,0,0).crossProduct(&Vec3f::new_i32(0,0,1)));
    }

    #[test]
    fn test_dotProduct() {
        assert_eq!(12.0, Vec3f::new_i32(1,2,3).dotProduct(&Vec3f::new_i32(4, -5, 6)));
    }
}
