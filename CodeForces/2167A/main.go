package main

import "fmt"

func main() {
	var t int
	fmt.Scanf("%d\n", &t)
	for range t {
		var a, b, c, d int
		fmt.Scanf("%d %d %d %d\n", &a, &b, &c, &d)
		if a == b && b == c && c == d {
			fmt.Println("YES")
		} else {
			fmt.Println("NO")
		}
	}
}
