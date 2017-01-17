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
        let normal = v0v1.crossProduct(&v0v2);

        // check if parallel
        let planeDirAngle = normal.dotProduct(direction);
        if planeDirAngle.abs() < EPSILON {
            //approx. parallel, on intersection
            return false;
        }

        // TODO http://www.scratchapixel.com/code.php?id=9&origin=/lessons/3d-basic-rendering/ray-tracing-rendering-a-triangle

        return false;
    }
}
