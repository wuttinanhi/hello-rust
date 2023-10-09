pub fn modify_box_test() {
    let mut numbox = Box::new(10);

    // modify numbox value to +10
    *numbox += 10;

    println!("numbox: {}", numbox);
}
