// Oliver maskerys beautifully concise method.

fn main(){

    let result: u32 = (0..1000).filter(|x| (x%5 == 0) || (x%3 == 0)).sum();
    println!("Total values of multiples of 3 and 5 from 1 to 1000: {}",result);
}
