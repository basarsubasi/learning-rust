
fn main() {
    let mut counter:i32 = 0;

    let result:i32 = loop {
        counter += 1;

        if counter < 10 {
            //println!("{counter}")
        }

        if counter ==10 {
            break counter*2; // break is similar to return, break only exits the current loop, while return exits the current function all together
        }
    };

    // println!("{result}");


    let mut count:i32 = 0;

    'counting_up: loop {
        println!("count={count}");

        let mut remaining:i32 = 10;

        'subtract_one_from_remaining: loop {
            println!("remaining = {remaining}");

            if remaining == 3 {
                break 'subtract_one_from_remaining ;
            }

            if count == 5 {
                break 'counting_up;
                
            }

            remaining -= 1;

        }
        count +=1;

    }

    println!("end count = {count}")

}
