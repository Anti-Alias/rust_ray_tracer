use vec::{Vec3, Ray3, Plane};

trait SceneObject {

}

struct Scene {
    pub objects: Vec<SceneObject>
}