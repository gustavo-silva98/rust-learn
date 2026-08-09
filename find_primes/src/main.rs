use std::{env, time::Instant};

fn main() {
    let start = Instant::now();
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        panic!("Bota o número de primos ae pae!");
    }
    let cap = args[1].parse::<usize>().unwrap();
    let mut primes = vec![0; cap];
    let mut next_prime_index = 0;
    if next_prime_index < cap {
        primes[0] = 2;
        next_prime_index += 1;
    }
    let mut i = 3;
    while next_prime_index < cap {
        if is_prime(&primes[..next_prime_index], i) {
            primes[next_prime_index] = i;
            next_prime_index += 1;
        }
        i += 2;
    }
    let duration = start.elapsed();
    println!("{:?}", primes);
    println!("Tempo decorrido {:?}", duration);
}

fn is_prime(array_primes: &[isize], num: isize) -> bool {
    for item in array_primes {
        if item * item > num {
            return true;
        } else if num % item == 0 {
            return false;
        }
    }
    return true;
}
