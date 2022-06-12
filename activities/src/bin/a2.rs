// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn main() {
    let s = sum(1,2);
    show(s);
}

fn show(a:isize) {
    println!("{:?}",a)
}

fn sum(a: isize, b: isize) -> isize {
    a + b
}
