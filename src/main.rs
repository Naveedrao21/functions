fn main() {
    println!("Main function.");
    another_function(5);
    print_labeled_measurement(5, 'h');
}

fn another_function(x: u32) {
    println!("The value of x is : {x}");
}
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

