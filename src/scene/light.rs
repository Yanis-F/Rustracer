#[derive(Clone)]
pub enum SceneLight {
    Directional(light_type::Directional),
    Point(light_type::Point),
}

pub mod light_type {

    use crate::color::*;
    use crate::math::*;

    #[derive(Clone)]
    pub struct Directional {
        pub color: Rgb,
        /// /!\ not normalized
        pub direction: Vector3,
    }
    impl Default for Directional {
        fn default() -> Self {
            Self {
                color: Rgb::default(),
                direction: vec3_neg(VECTOR3_UP),
            }
        }
    }

    #[derive(Clone, Default)]
    pub struct Point {
        pub color: Rgb,
        pub position: Vector3,
    }
}
