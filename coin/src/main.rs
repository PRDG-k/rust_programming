fn main() {
    let price = 3950;
    let n_coin_500 = 10;
    let n_coin_100 = 3;
    let n_coin_50 = 10;

    fn calc(c500: i32, c100: i32, c50: i32) -> i32 {
        500 * c500 + 100 * c100 + 50 * c50
    }

    for i500 in 0..n_coin_500 + 1 {
        if calc(i500, 0, 0) > price {
            continue;
        }

        for i100 in 0..n_coin_100 + 1 {
            if calc(i500, i100, 0) > price {
                continue;
            }

            for i50 in 0..n_coin_50 + 1 {
                let total = calc(i500, i100, i50);

                if total > price {
                    continue;
                }

                if total == price {
                    println!("500x{}+100x{}+50x{}={}", i500, i100, i50, total);
                }
            }
        }
    }
}
