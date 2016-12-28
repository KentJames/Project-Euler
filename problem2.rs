//Project Euler Question 2
fn main() {
    // Defined upper bound.
    let upper_bound: i64 = 4000000;

    // We declare a mutable variable for the summed up fibonacci numbers.
    // Pass that mutable reference to the recursive fibonacci function..
    let mut complete_sum: i64 = 0;
    let final_sum = fibonacci(upper_bound,1,2,&mut complete_sum);
    println!("Largest Fibonacci number: {}",final_sum);
    println!("Final sum: {}",complete_sum+2);


}

// Took me a while to get this working as I had forgotten all about recursion.
// A much nicer way of doing these things :)
fn fibonacci(upperbound: i64, prev_2: i64, prev_1: i64, complete_sum: &mut i64)-> i64 {

    
    let current: i64 = prev_2 + prev_1;
    if current%2 ==0{
        *complete_sum = *complete_sum + current;
    }

    println!("{}",current);
    if current<=upperbound {
    
        return fibonacci(upperbound, prev_1, current,complete_sum);
        
    }
    else{
        return current;
    }


    

}
