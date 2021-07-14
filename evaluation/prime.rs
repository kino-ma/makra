fn is_prime(n: usize) -> bool {
    let mut i = 1;
    loop {
        i += 1;
        if n % i == 0 {
            break;
        }
    }

    return n == i;
}
