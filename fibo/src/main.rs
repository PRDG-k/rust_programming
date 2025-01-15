use std::collections::HashMap;

fn main() {
    let mut memo = HashMap::new();

    memo.insert(0, 1);
    memo.insert(1, 2);
    println!("{}", memo.get(&0).unwrap());
    println!("{}", memo.get(&1).unwrap());

    for i in 2..31 {
        memo.insert(i, memo.get(&(i - 2)).unwrap() + memo.get(&(i - 1)).unwrap());
        println!("{}", memo.get(&i).unwrap());
    }
}
