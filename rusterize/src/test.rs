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
fn test_vector_math_sub (){
    let a = Vec3 { x:1., y:0., z:5.};
    let b = Vec3 { x:1.5, y:2., z:-10.};
    let expected = Vec3 {x:-0.5, y:-2., z:15.};
    assert_eq!(a - b,expected);
}

#[test]
fn test_vector_math_mul (){
    let a = Vec3 { x:1., y:0., z:5.};
    let b = Vec3 { x:1.5, y:2., z:-10.};
    let expected = Vec3 {x:1.5, y:0., z:-50.};
    assert_eq!(a * b,expected);
}

#[test]
fn test_vector_math_div (){
    let a = Vec3 { x:1., y:0., z:5.};
    let b = Vec3 { x:2., y:2., z:10.};
    let expected = Vec3 {x:0.5, y:0., z:0.5};
    assert_eq!(a / b,expected);
}

#[test]
fn test_vector_math_div_by_zero (){
    let a = Vec3 { x:1., y:0., z:5.};
    let b = Vec3 { x:2., y:2., z:0.};
    let result = std::panic::catch_unwind(|| a / b);
    assert!(result.is_err()); 
}

#[test]
fn test_vector_math_cross (){
    let a = Vec3 { x:1., y:2., z:5.};
    let b = Vec3 { x:5., y:1., z:3.};
    let expected = Vec3 {x:1., y:22., z:-9.};
    assert_eq!(a.cross(&b),expected);
}

#[test]
fn test_vector_math_scale (){
    let a = Vec3 { x:1., y:2., z:5.};
    let factor = 10.5;
    let expected = Vec3 {x:10.5, y:21., z:52.5};
    assert_eq!(a.scale(factor),expected);
}

#[test]
fn test_vector_math_dot (){
    let a = Vec3 { x:3., y:2.2, z:8.};
    let b = Vec3 { x:5., y:1., z:3.};
    let expected: f32 = 41.2    ;
    assert!( ulps_eq!(a.dot(&b),expected, max_ulps=2 ) );

    let a = Vec3 { x:1., y:1., z:0.};
    let b = Vec3 { x:0., y:0., z:1.};
    let expected: f32 = 0.;
    assert!( ulps_eq!(a.dot(&b),expected, max_ulps=2 ) );

    let a = Vec3 { x:1., y:1., z:0.};
    let b = Vec3 { x:0., y:1., z:0.};
    let expected: f32 = 1.;
    assert!( ulps_eq!(a.dot(&b),expected, max_ulps=2 ) );
}

#[test]
fn test_vector_math_length (){
    let a = Vec3 { x:1., y:0., z:0.};
    let expected: f32 = 1.;
    assert!( ulps_eq!(a.length(),expected, max_ulps=2 ) );

    let a = Vec3 { x:1., y:1., z:0.};
    let expected: f32 = 2_f32.sqrt();
    assert!( ulps_eq!(a.length(),expected, max_ulps=2 ) );
}

#[test]
fn test_vector_math_norm (){
    let a = Vec3 { x:1., y:1., z:0.};
    let b = Vec3 { x:1_f32/2_f32.sqrt(), y:1_f32/2_f32.sqrt(), z:0_f32};
    assert_eq!( a.norm(), b );
}

#[test]
fn test_std_trait_add (){
    let a = Vec3 { x:1., y:0., z:5.};
    let b = Vec3 { x:1.5, y:2., z:-10.};
    let expected = Vec3 {x:2.5, y:2., z:-5.};
    assert_eq!(a + b,expected);
}

#[test]
fn test_std_trait_sub (){
    let a = Vec3 { x:1., y:0., z:5.};
    let b = Vec3 { x:1.5, y:2., z:-10.};
    let expected = Vec3 {x:-0.5, y:-2., z:15.};
    assert_eq!(a - b,expected);
}

#[test]
fn test_std_trait_mul (){
    let a = Vec3 { x:1., y:0., z:5.};
    let b = Vec3 { x:1.5, y:2., z:-10.};
    let expected = Vec3 {x:1.5, y:0., z:-50.};
    assert_eq!(a * b,expected);
}

#[test]
fn test_std_trait_div (){
    let a = Vec3 { x:1., y:0., z:5.};
    let b = Vec3 { x:2., y:2., z:10.};
    let expected = Vec3 {x:0.5, y:0., z:0.5};
    assert_eq!(a / b,expected);
}

#[test]
fn test_std_trait_div_by_zero (){
    let a = Vec3 { x:1., y:0., z:5.};
    let b = Vec3 { x:2., y:2., z:0.};
    let result = std::panic::catch_unwind(|| a / b);
    assert!(result.is_err()); 
}