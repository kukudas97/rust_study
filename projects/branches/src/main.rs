fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let a_box = Box::new(5);
    println!("{}", a_box);
}
