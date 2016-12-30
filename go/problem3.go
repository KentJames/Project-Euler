package main

import (
    "fmt"
    "strconv"
)

func main(){

    var prime_number string
    //var prime_factors []int

    fmt.Printf("Please enter a number you want to prime factor: ")
    fmt.Scanln(&prime_number)

    prime_number_num, err := strconv.Atoi(prime_number)
    if err == nil{
        fmt.Printf("Number entered: %d\n",prime_number_num)
    }
    original_prime := prime_number_num

    a:=1
    for a==1{
        for 



    }



}
