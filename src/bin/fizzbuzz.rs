fn main() {
    // loop through 1-100
    // if divisible by 3 print fizz
    // if divisible by 5 print buzz
    // if divisible by 3 and 5 print fizzbuzz
    for n in 1..=100 {
        println!("number: {n} ");
        if n % 3 == 0 && n % 5 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        }
    }
}
