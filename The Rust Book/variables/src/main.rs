fn main() {
    let mut x = 5;
    println!("the value of x is {x}");
    x = 6;
    println!("the value of x is {x}");

    let y = 5;

    let y = y+1;

    {
        let y = y*2;

        println!("the value of y is {y}");
    }

    println!("the value of y is {y}");
}
