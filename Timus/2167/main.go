package main

import "fmt"

const N = 1500000

func main() {
	primes := make([]bool, N)
	for i := 2; i < N; i++ {
		primes[i] = true
	}
	for i := 2; i < N; i++ {
		if !primes[i] {
			continue
		}
		for j := 2; i*j < N; j++ {
			primes[i*j] = false
		}
	}

	var t int
	fmt.Scanf("%d\n", &t)
	for i := 0; i < t; i++ {
		var x int
		fmt.Scanf("%d\n", &x)
		var ok bool
		for j := 0; j < N; j++ {
			if x^j < N && primes[x^j] {
				fmt.Printf("%d\n", j)
				ok = true
				break
			}
		}
		if !ok {
			panic("answer not found")
		}
	}
}
