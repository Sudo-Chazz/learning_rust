use std::io;

fn main() {
    loop{
        println!("Enter a number to find the result of it in the fibonacci sequence.");
        let mut n = String::new();
        io::stdin()
            .read_line(&mut n)
            .expect("failed to readline.");

        match n.trim().parse::<u64>() {
            Ok(num) => break println!("{}", gen_fib(num)),
            Err(_) => println!("\n\x1b[1;31mPositive integers only please.\x1b[0m\n"),
        };
    }   
}

fn gen_fib(depth: u64) -> u64 {
    match depth {
        0 => 1,
        1 => 1,
        _ => gen_fib(depth - 1) + gen_fib(depth - 2),
        // due to limitations of recursion high numbers like 100 will cause issues. 
    }
}