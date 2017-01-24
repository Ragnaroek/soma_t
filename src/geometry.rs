use vector::Vec3f;

pub trait Object {
    fn intersect(&self, orig: &Vec3f, direction: &Vec3f) -> bool;
}

pub struct Triangle {
    v0: Vec3f,
    v1: Vec3f,
    v2: Vec3f
}

const EPSILON:f32 = 1e-8;

impl Triangle {
    pub fn new(v0: Vec3f, v1: Vec3f, v2: Vec3f) -> Triangle {
        return Triangle{v0: v0, v1: v1, v2: v2};
    }
}

impl Object for Triangle {

    fn intersect(&self, orig: &Vec3f, direction: &Vec3f ) -> bool {

        // TODO optimisation: compute the normal once in Triange constructor!!
        // compute plane's normal
        let v0v1 = &self.v1 - &self.v0;
        let v0v2 = &self.v2 - &self.v0;
        let normal = v0v1.cross_product(&v0v2);

        // check if parallel
        let plane_dir_angle = normal.dot_product(direction);
        if plane_dir_angle.abs() < EPSILON {
            //approx. parallel, on intersection
            println!("parallel!");
            return false;
        }
        let d = normal.dot_product(&self.v0);
        let t = (normal.dot_product(direction) + d) / plane_dir_angle;

        if t.is_sign_negative() { // check if behind the ray
            println!("behind ray! {:?}", t);
            return false
        }

        // point on triangle plane
        let p = orig + &(direction * t);

        // inside-out-test

        let edge0 = v0v1;
        let vp0 = &p - &self.v0;
        let c1 = edge0.cross_product(&vp0);
        if normal.dot_product(&c1).is_sign_negative() {
            return false;
        }

        let edge1 = &self.v2 - &self.v1;
        let vp1 = &p - &self.v1;
        let c2 = edge1.cross_product(&vp1);
        if normal.dot_product(&c2).is_sign_negative() {
            return false;
        }

        let edge2 = &self.v0 - &self.v2;
        let vp2 = &p - &self.v2;
        let c3 = edge2.cross_product(&vp2);
        if normal.dot_product(&c3).is_sign_negative() {
            return false;
        }

        return true;
    }
}
