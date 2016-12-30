package main

import (
    "fmt" 
)


func main(){

    var x, multiples_sum int

    for x < 1000 {
        if(x%5==0)||(x%3==0){
            fmt.Printf("Multiple of 5 or 3 found: %d\n",x)
            multiples_sum += x
        }
        x += 1

    }
    fmt.Printf("Total of all multiples: %d\n",multiples_sum)

}
