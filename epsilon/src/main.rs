fn main() {
    let result: f32 = 0.1+0.1;
    let desired: f32 = 0.2;
    let absolute_diff = (desired - result).abs();
    assert!(absolute_diff <= f32::EPSILON);
}
