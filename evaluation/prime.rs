fn main() {
    let n = 32749;
    let mut i = 1;
    loop {
        i += 1;
        if n % i == 0 {
            break;
        }
    }

    println!("function result: {}", n == i);
}
