pub fn string_test() {
    let mut x = String::from("Hello World!");
    change_str(&mut x);
    println!("{}", x);

    let mut y = String::from("Hello World!");
    change_str_v2(&mut y);
    println!("{}", y);

    let add = |a: i32, b: i32| a + b;

    let mut result = 0;

    for x in 1..=5 {
        print!("{}", x);
        result = add(result, x);
    }

    println!();

    println!("result = {}", result)
}

pub fn change_str(str: &mut String) {
    *str = str.replace("World", "Rust").to_string();
}

pub fn change_str_v2(original: &mut String) {
    let str = original;
    *str = str.replace("World", "from change_str_v2");
}
