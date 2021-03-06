use std::env;

fn main() {
    //Get range from command line (primes in range 1..input)
    let num = env::args().nth(1).unwrap();

    let max: usize = match num.parse() {
        Ok(n) => {
            n
        },
        Err(_) => {
            println!("Error: not a valid number!");
            return;
        }
    };

    let mut primes = Vec::with_capacity(max);
    let mut items_pushed = 0;
    loop {
        primes.push(true);
        items_pushed += 1;
        if items_pushed == max {
            break;
        }
    }

    primes[0] = false;
    if max > 1 {
        primes[1] = false;
    }

    let mut prime_count = max - 2;
    for i in 0..(max as f64).sqrt() as usize {
        if primes[i] {
            let mut mult = i * i;
            while mult < max {
                if primes[mult] {
                    prime_count -= 1;
                    primes[mult] = false;
                }
                mult += i;
            }
        }
    }

    println!("Number of primes: {}", prime_count)
}
