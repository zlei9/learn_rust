pub mod module2;

pub fn print_a_Z(){
    for a in ('Z'..='a').rev() {
        println!("{a}")
    }
}