package main

import "fmt"

var (
	n int
	m [N][N]int
)

func main() {
	fmt.Scanf("%d\n", &n)

	for i := 0; i < n; i++ {
		x := 2*n + 1
		c := 2*n + 1 - i
		for r := 2 * n; r >= 1; r-- {
			m[r-1][c-1] = x
			x--
		}
	}
}
