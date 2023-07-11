pub mod module1;

fn main() {
    let mut a1 = String::from("I love Rust");
    let a2 = &mut a1;
    *a2 = String::from("I love Rust!");
    let a3 = a2;
    foo(a3);
    // let a4 = &a2;
    // let a5 = a2;

    println!("{a3}");
    println!("{a1}");

    // println!("{a3}");
    // println!("{a4}");
    // println!("{a5}");
    // println!("start a-Z");
    // module1::print_a_Z();
    // println!("start A-z");
    // module1::module2::print_A_z();
}

fn foo(s: &mut String) {
    s.push_str(" too!")
}
