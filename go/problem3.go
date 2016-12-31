package main

import (
    "fmt"
    "strconv"
)

func main(){

    var prime_number string
    var prime_factors []int

    fmt.Printf("Please enter a number you want to prime factor: ")
    fmt.Scanln(&prime_number)

    prime_number_num, err := strconv.Atoi(prime_number)
    if err == nil{
        fmt.Printf("Number entered: %d\n",prime_number_num)
    }
    original_prime := prime_number_num

    //Loop indices...


    for {
        var i int
        for  i=2; i<=5000000;i++{
            if prime_number_num%i == 0{
                fmt.Printf("Prime factor found: %d\n",i)
                prime_number_num = prime_number_num/i;
                prime_factors = append(prime_factors,i)
                break
            } else {
                continue
            }

        }
        var check_var int = 1

        for num := range prime_factors {

            check_var = check_var * prime_factors[num]
        }
        if check_var == original_prime{
            fmt.Printf("Looks like we found all the prime factors! \n")
            break
        }



    }



}
