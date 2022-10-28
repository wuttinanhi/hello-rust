fn main() {
    let x = String::from("Hello Rust!");
    println!("{}", x);

    let add = |a: i32, b: i32| a + b;

    let mut result = 0;

    for x in 1..10 {
        println!("{}", x);
        result = add(result, x);
    }

    println!("result = {}", result)
}
