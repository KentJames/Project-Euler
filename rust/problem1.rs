fn main() {
    
    let upper_bound = 1000;
    let multiples = vec![3,5];

    let mut x = 0;
    let mut multiples_sum: i32 = 0; //Initialize empty number.
    
    while x < upper_bound {
        
        

    
        if (x%5==0)||(x%3==0) {

            
            println!("Multiple of 5/3 found: {}",x);
            multiples_sum = multiples_sum + x;
            
        
        

        }
        

        x = x+1;
    }
    println!("Total value of multiples found: {}",multiples_sum);
}
