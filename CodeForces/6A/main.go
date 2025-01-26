package main

import "fmt"

func Combine(a, b, c, d int, f func(int, int, int) bool) bool {
	return f(a, b, c) || f(a, b, d) || f(a, c, d) || f(b, c, d)
}

func Triangle(a, b, c int) bool {
	return (a+b > c) && (a+c > b) && (b+c > a)
}

func Segment(a, b, c int) bool {
	return (a+b == c) || (a+c == b) || (b+c == a)
}

func main() {
	var a, b, c, d int
	fmt.Scanf("%d %d %d %d", &a, &b, &c, &d)
	if Combine(a, b, c, d, Triangle) {
		fmt.Println("TRIANGLE")
	} else if Combine(a, b, c, d, Segment) {
		fmt.Println("SEGMENT")
	} else {
		fmt.Println("IMPOSSIBLE")
	}
}
