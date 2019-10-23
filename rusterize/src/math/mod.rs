pub mod vectors {
    
    use std::ops::Add;

   
    #[derive(Debug, PartialEq)]
    pub struct Vec3 {
        pub x: f32,
        pub y: f32,
        pub z: f32,
    } 
    
    pub trait VectorMath{
        fn add(self, other: &Self) -> Self;
    } 

    impl VectorMath for Vec3 {
        fn add(self, other: &Self) -> Self{
            Self {
               x: self.x + other.x,
               y: self.y + other.y,
               z: self.z + other.z,
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
    
}