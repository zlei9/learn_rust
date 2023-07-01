pub mod module1;

fn main() {
    println!("start a-Z");
    module1::print_a_Z();
    println!("start A-z");
    module1::module2::print_A_z();
}
