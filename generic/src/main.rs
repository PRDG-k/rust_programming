// 덧셈 트레이트를 지원하는 제네릭 타입만을 인자로 사용하는 방법
fn add<T: std::ops::Add<Output = T>>(i: T, j: T) -> T{
    i + j
}

fn main() {
    
}
