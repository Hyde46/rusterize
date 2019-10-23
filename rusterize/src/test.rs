use crate::math::vectors::Vec3;
use crate::math::vectors::VectorMath;

#[test]
fn test_vector_math_add (){
    let a = Vec3 { x:1., y:0., z:5.};
    let b = Vec3 { x:1.5, y:2., z:-10.};
    let expected = Vec3 {x:2.5, y:2., z:-5.};
    assert_eq!(a.add(&b),expected);
}

#[test]
fn test_std_trait_add (){
    let a = Vec3 { x:1., y:0., z:5.};
    let b = Vec3 { x:1.5, y:2., z:-10.};
    let expected = Vec3 {x:2.5, y:2., z:-5.};
    assert_eq!(a + b,expected);
}
