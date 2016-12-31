// Finds smallest multiple evenly divisible by a number!
//

use std::io;

fn main() {

    let mut highest_divisor = String::new();
    println!("Please enter highest divisor: ");
    io::stdin().read_line(&mut highest_divisor)
        .expect("Failed to read line.");
    let highest_divisor: i32 = highest_divisor.trim().parse()
        .expect("Not a number!");
    println!("Highest divisor: {}",highest_divisor);

    let mut i: i32 = highest_divisor+1;
    loop {
        let mut smallest_mult: bool = false;
        for j in 1..(highest_divisor+1) {
            
            let rem = i%j;
            if rem >= 1 {
                smallest_mult = false;
                break;
            } 
            else {
                smallest_mult = true
            };
        }
        if smallest_mult == true{
            println!("Smallest multiple found: {}",i);
            break;
        }
        i+= 1;



    }



}
