pub mod vectors {
    
    use std::ops::{Add, Sub, Div, Mul};

   
    #[derive(Debug, PartialEq)]
    pub struct Vec3 {
        pub x: f32,
        pub y: f32,
        pub z: f32,
    } 
    
    pub trait VectorMath{
        fn add(self, other: &Self) -> Self;
        fn sub(self, other: &Self) -> Self;
        fn mul(self, other: &Self) -> Self;
        fn div(self, other: &Self) -> Self;
    } 

    impl VectorMath for Vec3 {
        fn add(self, other: &Self) -> Self{
            Self {
               x: self.x + other.x,
               y: self.y + other.y,
               z: self.z + other.z,
           }
        }
        fn sub(self, other: &Self) -> Self{
            Self {
               x: self.x - other.x,
               y: self.y - other.y,
               z: self.z - other.z,
           }
        }
        fn mul(self, other: &Self) -> Self{
            Self {
               x: self.x * other.x,
               y: self.y * other.y,
               z: self.z * other.z,
           }
        }
        fn div(self, other: &Self) -> Self{
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
}