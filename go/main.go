package main
import ("fmt")

func main() {
   var divisor float64 = 1
   var result float64 = 0

   for i:=0; i < 1000000000; i++ {

       var sub_result = 4 / divisor

       if i % 2 == 0 {
            result += sub_result
        } else {
            result -= sub_result
        }

        divisor += 2
    }
    fmt.Printf("%.50f", result)
}
