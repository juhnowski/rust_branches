fn main() {
    let a = 9;

    if a < 10 {
        println!("a={} < 10", a);
    } else {
        println!("a={} > 10", a);
    }

    if a % 2 == 0 {
        println!("a={} is odd", a );
    } else if a % 3 == 0 {
        println!("a={} divided by 3", a );
    }
}
