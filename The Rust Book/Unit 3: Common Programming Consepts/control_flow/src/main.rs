fn main() {
    let number: i32 = 7;

    if number < 6 {
        println!("number is smaller than 6");
    } else {
        println!("number is not smaller than 6");
    }

    let condition:bool = false;

    let x:i32 = if condition {1} else {2};

    println!("{x}");
}
