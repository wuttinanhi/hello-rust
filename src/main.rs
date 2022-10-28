fn main() {
    let mut x = String::from("Hello World!");

    change_str(&mut x);

    println!("{}", x);
    println!("{}", &x);
    println!("{}", &mut x);

    let add = |a: i32, b: i32| a + b;

    let mut result = 0;

    for x in 1..5 {
        print!("{}", x);
        result = add(result, x);
    }

    println!();

    println!("result = {}", result)
}

pub fn change_str(str: &mut String) {
    *str = (*str).replace("World", "Rust").to_string();
}
