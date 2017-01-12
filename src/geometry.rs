use vector::Vec3f;

trait Object {
    fn intersect(self, orig: &Vec3f, direction: &Vec3f ) -> bool;
}

struct Triangle {
    v0: Vec3f,
    v1: Vec3f,
    v2: Vec3f
}

impl Object for Triangle {
    fn intersect(self, orig: &Vec3f, direction: &Vec3f ) -> bool {

        // compute plane's normal
        let v0v1 = self.v1 - &self.v0;
        let v0v2 = self.v2 - &self.v0;

        //Vec3f N = v0v1.crossProduct(v0v2); // N
        //float denom = N.dotProduct(N);


        return false;
    }
}
