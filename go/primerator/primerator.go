package main

import (
	"fmt"
	"math"
	"os"
	"strconv"
)

func is_prime(x int) bool {
	var ret bool = true
	var limit int = int(math.Sqrt(float64(x))) + 1
	for check := 2; check < limit; check++ {
		if x%check == 0 {
			ret = false
			break
		}
	}
	return ret
}

func main() {
	var cli_args []string = os.Args
	if len(cli_args) < 2 {
		fmt.Println("Usage: primerator <number>")
		os.Exit(1)
	} else {
		prime_count, err := strconv.Atoi(cli_args[1])
		if err != nil {
			fmt.Println("Error: Argument is not an integer.")
			os.Exit(1)
		}
		fmt.Printf("Finding %v primes \n", prime_count)
		var primes []int64
		primes = append(primes, 2)
		fmt.Printf("%v \n", len(primes))
		var to_test int64 = 3
		for prime_count > len(primes) {
			if is_prime(int(to_test)) {
				primes = append(primes, to_test)
			}
			to_test += 2
		}
		fmt.Printf("%v \n", primes[0])
		fmt.Printf("%v \n", primes[prime_count-1])
	}
}
