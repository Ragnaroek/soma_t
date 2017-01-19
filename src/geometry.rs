use vector::Vec3f;

trait Object {
    fn intersect(self, orig: &Vec3f, direction: &Vec3f ) -> bool;
}

struct Triangle {
    v0: Vec3f,
    v1: Vec3f,
    v2: Vec3f
}

const EPSILON:f32 = 1e-8;

impl Object for Triangle {
    fn intersect(self, orig: &Vec3f, direction: &Vec3f ) -> bool {

        // TODO optimisation: compute the normal once!
        // compute plane's normal
        let v0v1 = self.v1 - &self.v0;
        let v0v2 = self.v2 - &self.v0;
        let normal = v0v1.cross_product(&v0v2);

        // check if parallel
        let plane_dir_angle = normal.dot_product(direction);
        if plane_dir_angle.abs() < EPSILON {
            //approx. parallel, on intersection
            return false;
        }
        let d = normal.dot_product(&self.v0);
        let t = (normal.dot_product(direction) + d) / plane_dir_angle;

        if t.is_sign_negative() { // check if behind the ray
            return false
        }

        // point on triangle plane
        let p = orig + &(direction * t);

        // TODO inside out test!

        // TODO http://www.scratchapixel.com/code.php?id=9&origin=/lessons/3d-basic-rendering/ray-tracing-rendering-a-triangle

        return false;
    }
}
