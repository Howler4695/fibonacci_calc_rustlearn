use std::io;

fn main() {
    loop {

        println!("Calculate fibonacci to n: ");

        let mut fibonacci_n = String::new();

        io::stdin().read_line(&mut fibonacci_n).expect("Failed to read line!");

        let fibonacci_n: u64 = match fibonacci_n.trim().parse() {
            Ok(n) => n ,
            Err(_) => {
                println!("Not acceptable n\n---------------------");
                continue;
            }
        };

        let mut second_previous: u128 = 0;
        let mut first_previous: u128 = 1;

        for _ in 3..=fibonacci_n {
            let hold_first_previous = first_previous;
            first_previous += second_previous;
            second_previous = hold_first_previous;
        }

        println!("Fibonacci({fibonacci_n}) = {first_previous}");
    }
}
