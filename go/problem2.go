package main

import "fmt"

func main(){
    var upperbound int = 4000000
    var complete_sum int = 0

    fibonacci(upperbound,1,2,&complete_sum)
    fmt.Printf("Final Sum: %d\n",complete_sum+2)


}

func fibonacci(upperbound int, prev_2 int, prev_1 int, complete_sum *int) int {

    var current = prev_2 + prev_1
    if current%2==0{
        *complete_sum = *complete_sum + current;
    }

    fmt.Printf("%d\n",current)

    if current<=upperbound{
        return fibonacci(upperbound,prev_1,current,complete_sum)
    } else {
        return current
    }

}




