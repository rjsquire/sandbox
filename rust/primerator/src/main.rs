use std::io;

fn main() {
    let prime_count: i64 = get_prime_count();
    let mut primes: Vec<i64> = Vec::new();
    primes.push(2);
    let mut to_test: i64 = 3;
    while  prime_count > primes.len() as i64 {
        if is_prime(to_test) {
            // println!("{to_test} is prime");
            primes.push(to_test);
            // println!("{:?}", primes.len());
        }
        to_test += 2;
    }
    println!("{:?}", primes);
}

fn get_prime_count() -> i64 {
    println!("How many primes do you want!");
    let mut prime_count = String::new();
    loop {    
        println!("Please input an integer number of primes you want:");

        io::stdin()
            .read_line(&mut prime_count)
            .expect("Failed to read line");

        let prime_count: i64 = match prime_count.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You entered: {prime_count}");
        return prime_count;
    }
}

fn is_prime(x: i64) -> bool {
    // let mut keep_going: bool = true;
    let mut ret: bool = true;
    let _limit: i64 = f64::sqrt(x as f64) as i64 + 1;
    let mut check: i64 = 2;
    // println!("{x} passed to is_prime");
    while check < _limit {
        // println!("Currently checking if {x} % {check} == 0");
        // println!("{:?}", x % check);
        if x % check == 0 || check > _limit {
            ret = false;
            break;
        } 
        else {
            check += 1;
        }
    }
    return ret;
}