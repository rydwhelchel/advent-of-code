package main

import "fmt"


func main() {
    a := 'a'
    b := 'A'

    z := 'z'
    Z := 'Z'
    println(a)
    println(b)

    println("Value of 'a' is: " + fmt.Sprint(calcValue(a)))
    println("Value of 'A' is: " + fmt.Sprint(calcValue(b)))
    println("Value of 'z' is: " + fmt.Sprint(calcValue(z)))
    println("Value of 'Z' is: " + fmt.Sprint(calcValue(Z)))
}

func calcValue(r rune) int {
    if r > 96 {
        return int(r) - 96
    }
    return int(r) - 64 + 26
}
