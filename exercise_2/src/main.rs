fn fibonacci(n: u32) -> u64 {
    if n == 0 {
        return 0;
    }

    let mut a = 0;
    let mut b = 1;
    for _ in 2..n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    b
}

fn main() {
    for i in 0..10 {
        println!("Fibonacci({}) = {}", i, fibonacci(i))
    }
}
