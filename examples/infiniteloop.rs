use std::process;

fn main() {
    println!("My PID is {}", process::id());

    loop {}
}