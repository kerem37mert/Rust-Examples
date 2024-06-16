fn primeNumber(number: i16) -> bool {
    let mut i: i16 = 2;

    while i < number {
        if number % i == 0 {
            return false;
        }

        i += 1;
    }

    return true;
}

fn main() {
    let mut number = 7;

    if primeNumber(number) {
        println!("{0} is a prime number", number);
    } else {
        println!("{0} is not a prime number", number);
    }
}
