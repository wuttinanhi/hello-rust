mod box_test;
mod concurrent_test;

fn main() {
    concurrent_test::multitest();

    box_test::modify_box_test();
}
