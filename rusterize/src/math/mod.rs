pub mod geometry;
pub mod matrices;
pub mod vectors {

    use std::f32;
    use std::ops::{Add, Div, Mul, Sub};

    #[derive(Debug, PartialEq)]
    pub struct Vec3 {
        pub x: f32,
        pub y: f32,
        pub z: f32,
    }

    impl Vec3 {
        pub fn empty() -> Self {
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }
        }
        pub fn new(x: f32, y: f32, z: f32) -> Self {
            Vec3 { x, y, z }
        }
    }

    pub trait VectorMath {
        fn add(&self, other: &Self) -> Self;
        fn sub(&self, other: &Self) -> Self;
        fn mul(&self, other: &Self) -> Self;
        fn div(&self, other: &Self) -> Self;
        fn cross(&self, other: &Self) -> Self;
        fn scale(&self, scalar: f32) -> Self;
        fn dot(&self, other: &Self) -> f32;
        fn length(&self) -> f32;
        fn norm(&self) -> Self;
    }

    impl VectorMath for Vec3 {
        fn length(&self) -> f32 {
            (self.x.powf(2.) + self.y.powf(2.) + self.z.powf(2.)).sqrt()
        }
        fn dot(&self, other: &Self) -> f32 {
            self.x * other.x + self.y * other.y + self.z * other.z
        }
        fn add(&self, other: &Self) -> Self {
            Self {
                x: self.x + other.x,
                y: self.y + other.y,
                z: self.z + other.z,
            }
        }
        fn sub(&self, other: &Self) -> Self {
            Self {
                x: self.x - other.x,
                y: self.y - other.y,
                z: self.z - other.z,
            }
        }
        fn mul(&self, other: &Self) -> Self {
            Self {
                x: self.x * other.x,
                y: self.y * other.y,
                z: self.z * other.z,
            }
        }
        fn div(&self, other: &Self) -> Self {
            if other.x == 0. || other.y == 0. || other.z == 0. {
                panic!("Dividing by 0!")
            }
            Self {
                x: self.x / other.x,
                y: self.y / other.y,
                z: self.z / other.z,
            }
        }
        fn cross(&self, other: &Self) -> Self {
            Self {
                x: self.y * other.z - self.z * other.y,
                y: self.z * other.x - self.x * other.z,
                z: self.x * other.y - self.y * other.x,
            }
        }

        fn scale(&self, factor: f32) -> Self {
            Self {
                x: self.x * factor,
                y: self.y * factor,
                z: self.z * factor,
            }
        }
        fn norm(&self) -> Self {
            /*
            Normalize Vector
            n = u / length(u)
            */
            let length = self.length();
            self.scale(1_f32 / length)
        }
    }

    impl Add for Vec3 {
        type Output = Self;

        fn add(self, other: Self) -> Self {
            Self {
                x: self.x + other.x,
                y: self.y + other.y,
                z: self.z + other.z,
            }
        }
    }
    impl Sub for Vec3 {
        type Output = Self;

        fn sub(self, other: Self) -> Self {
            Self {
                x: self.x - other.x,
                y: self.y - other.y,
                z: self.z - other.z,
            }
        }
    }
    impl Mul for Vec3 {
        type Output = Self;

        fn mul(self, other: Self) -> Self {
            Self {
                x: self.x * other.x,
                y: self.y * other.y,
                z: self.z * other.z,
            }
        }
    }
    impl Div for Vec3 {
        type Output = Self;

        fn div(self, other: Self) -> Self {
            if other.x == 0. || other.y == 0. || other.z == 0. {
                panic!("Dividing by 0!")
            }
            Self {
                x: self.x / other.x,
                y: self.y / other.y,
                z: self.z / other.z,
            }
        }
    }

    pub fn normalize(vector: &Vec3) -> Vec3 {
        vector.norm()
    }
}
