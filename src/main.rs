fn main() {
    println!("Hello, world!");
    fizz_buzz();
}

fn fizz_buzz() {
    let mut count = 0;
    for n in 1..302 {
        match (n % 3, n % 5) {
            (0,0) => {
                println!("fizzbuzz");
                count+=1;
            },
            (0,_) => println!("fizz"),
            (_,0) => println!("buzz"),
            _ => println!("{}", n)
        }
    }
    println!("Amount of fizz buzz {}", count);
}
