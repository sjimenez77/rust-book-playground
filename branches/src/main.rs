fn main() {
    let fib = 80;
    let mut prev: u64 = 0;
    let mut current: u64 = 1;

    println!("The fibonacci number 1 is: {}", prev);
    for x in 1..fib {
        println!("The fibonacci number {} is: {}", x+1, current);
        let temp = current;
        current = current + prev;
        prev = temp;
    }
}