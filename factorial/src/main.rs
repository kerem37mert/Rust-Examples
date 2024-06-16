fn factoial(number: i16) -> i16 {
    if number == 1 {
        return 1;
    } else {
        return number * factoial(number-1);
    }
}


fn main() {
    let mut number: i16 = 5;
    println!("{0}", factoial(number));
}