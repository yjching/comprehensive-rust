//
fn is_divisible(n: i32, divisor: i32) -> bool {
    n % divisor == 0
}


fn fizzbuzz(n: i32) -> String {
    //Check if multiple of 3 or 5 to fizz and/or buzz
    let fizz: &str = if is_divisible(n, 3) {"fizz"} else {""};
    let buzz: &str = if is_divisible(n, 5) {"buzz"} else {""};
    if fizz.is_empty() && buzz.is_empty() {
        return format!("{n}");
    }
    return format!("{fizz}{buzz}")
}

fn print_fizzbuzz (n: i32) {
    for i in 1..=n {
        println!("{}", fizzbuzz(i))
    }
}

// Run fizzbuzz
fn main() {
    print_fizzbuzz(200)
}
