// Finds highest palindromic number composed of two 3-digit numbers.

use std::io;

fn main(){

    let mut highest_palnum: i32 = 0;

    for i in 100..1000 {

        for j in 100..1000 {
            let product: i32 = i*j;
            if product == reverse_number(product){
                println!("Palindrominc number found: {}. Product of {} and {}",product,i,j);
                if(product > highest_palnum){
                    highest_palnum = product;
                }

            }

        }

    }

    println!("Highest palindromic number found: {}",highest_palnum);

}

fn reverse_number(number: i32) -> i32 {

    let mut rem;
    let mut sum = 0;
    let mut num = number;

    while num > 1 {
        rem = num % 10;
        sum = sum*10 + rem;
        num = num/10;

    }
    
    return sum;

}

