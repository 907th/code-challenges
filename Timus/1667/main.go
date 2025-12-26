package main

import (
	"fmt"
	"math"
)

const (
	N   = 20
	LIM = 250
)

type Ans [N][N]int

func squareRoot(x int) (int, bool) {
	if x <= 0 {
		panic("squareRoot")
	}
	r := int(math.Sqrt(float64(x)))
	if r*r < x {
		r++
	}
	if r*r > x {
		r--
	}
	return r, r*r == x
}

func isSquare(x int) bool {
	_, ok := squareRoot(x)
	return ok
}

func square(x int) int {
	return x * x
}

var smallNums = [][]int{
	{1},
	{3, 4},
	{2, 6, 9},
}

func findNums1(n int) []int {
	var res []int
	var sum int
	for a := 1; a <= n-3; a++ {
		res = append(res, a)
		sum += a * a
	}
	for x := n - 2; x < LIM; x++ {
		for y := x + 1; y < x+LIM; y++ {
			for z := y + 1; z < y+LIM; z++ {
				total := sum + x*x + y*y + z*z
				if isSquare(total) {
					res = append(res, x, y, z)
					return res
				}
			}
		}
	}
	panic("nums1 not found")
}

func noProblems(nums1, nums2 []int) bool {
	used := make(map[int]struct{})
	for _, n1 := range nums1 {
		for _, n2 := range nums2 {
			if _, ok := used[n1*n2]; !ok {
				used[n1*n2] = struct{}{}
			} else {
				return false
			}
		}
	}
	return true
}

func findNums2_2(nums1 []int) []int {
	for x := 1; x < LIM; x++ {
		for y := x + 1; y < x+LIM; y++ {
			total := x*x + y*y
			if isSquare(total) {
				res := []int{x, y}
				if noProblems(nums1, res) {
					return res
				}
			}
		}
	}
	panic("nums2_2 not found")
}

func findNums2(n int, nums1 []int) []int {
	res := []int{}
	var sum int
	a := 1
	for len(res) < n-3 {
		candidate := append(res, a)
		if noProblems(nums1, candidate) {
			res = candidate
			sum += a * a
		}
		a++
	}
	for x := a + 1; x < a+LIM; x++ {
		for y := x + 1; y < x+LIM; y++ {
			for z := y + 1; z < y+LIM; z++ {
				total := sum + x*x + y*y + z*z
				if isSquare(total) {
					candidate := append(res, x, y, z)
					if noProblems(nums1, candidate) {
						return candidate
					}
				}
			}
		}
	}
	panic("nums2 not found")
}

func findNums(r, c int) ([]int, []int) {
	var nums1 []int
	if r <= 3 {
		nums1 = smallNums[r-1]
	} else {
		nums1 = findNums1(r)
	}

	var nums2 []int
	switch c {
	case 1:
		nums2 = []int{1}
	case 2:
		nums2 = findNums2_2(nums1)
	default:
		nums2 = findNums2(c, nums1)
	}

	return nums1, nums2
}

func findAns(r, c int) Ans {
	nums1, nums2 := findNums(r, c)
	var ans Ans
	for i := 0; i < r; i++ {
		for j := 0; j < c; j++ {
			y1 := nums1[i]
			y2 := nums2[j]
			ans[i][j] = y1 * y1 * y2 * y2
		}
	}
	return ans
}

func printAns(r, c int, ans Ans) {
	for i := 0; i < r; i++ {
		for j := 0; j < c; j++ {
			if j > 0 {
				fmt.Print(" ")
			}
			fmt.Print(ans[i][j])
		}
		fmt.Println()
	}
}

func main() {
	var testsNum int
	fmt.Scanf("%d\n", &testsNum)
	for tn := 0; tn < testsNum; tn++ {
		var r, c int
		fmt.Scanf("%d %d\n", &r, &c)
		ans := findAns(r, c)
		if tn > 0 {
			fmt.Println()
		}
		printAns(r, c, ans)
	}
}
