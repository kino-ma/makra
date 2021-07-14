fn main() {
    let n = 2147483647;

    let mut i = 1;
    loop {
        i += 1;
        if n % i == 0  {
            break;
        }
    }

    println!("result: {}", n - i == 0);
}
