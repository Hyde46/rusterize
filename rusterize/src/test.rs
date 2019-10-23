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