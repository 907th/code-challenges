package main

import (
	"fmt"
	"strconv"
)

const N = 105

type (
	ints []int
	pair struct {
		p1 ints
		p2 ints
	}
)

func (p *pair) put(v int, z int) {
	if z > 0 {
		p.p1 = append(p.p1, v)
	} else {
		p.p2 = append(p.p2, v)
	}
}

func (a ints) String() string {
	var s string
	for i, v := range a {
		if i > 0 {
			s = s + " "
		}
		s = s + strconv.Itoa(v)
	}
	return s
}

var (
	n int
	m int
	g [N]ints

	x pair
	c [N]int
	p []pair

	dp [N][N]int
)

func coloring(v int, z int) bool {
	c[v] = z
	x.put(v, z)
	for i := 0; i < len(g[v]); i++ {
		u := g[v][i]
		if u == v {
			continue
		}
		if c[u] == 0 {
			coloring(u, -z)
		} else if c[u] == c[v] {
			return false
		}
	}
	return true
}

func main() {
	fmt.Scanf("%d %d\n", &n, &m)
	for i := 0; i < m; i++ {
		var a, b int
		fmt.Scanf("%d %d\n", &a, &b)
		g[a] = append(g[a], b)
		g[b] = append(g[b], a)
	}

	for u := 1; u <= 2*n; u++ {
		if c[u] != 0 {
			continue
		}
		x = pair{}
		if !coloring(u, 1) {
			fmt.Print("IMPOSSIBLE\n")
			return
		}
		p = append(p, x)
	}

	// validatePartitioning()

	dp[0][0] = 1
	for i := 0; i < len(p); i++ {
		x = p[i]
		for j := 0; j < N; j++ {
			if dp[i][j] == 0 {
				continue
			}
			dp[i+1][j+len(x.p1)] = 1
			dp[i+1][j+len(x.p2)] = 2
		}
	}

	if dp[len(p)][n] == 0 {
		fmt.Print("IMPOSSIBLE\n")
		return
	}

	x = pair{}
	j := n
	for i := len(p); i > 0; i-- {
		if dp[i][j] == 1 {
			x.p1 = append(x.p1, p[i-1].p1...)
			x.p2 = append(x.p2, p[i-1].p2...)
			j -= len(p[i-1].p1)
		} else {
			x.p1 = append(x.p1, p[i-1].p2...)
			x.p2 = append(x.p2, p[i-1].p1...)
			j -= len(p[i-1].p2)
		}
	}

	// validateFinalSolution(x)

	fmt.Printf("%s\n%s\n", x.p1.String(), x.p2.String())
}

func validateFinalSolution(t pair) {
	if len(t.p1) != n || len(t.p2) != n {
		panic("bad parts size")
	}

	var q [N]bool
	for _, v := range t.p1 {
		if v < 0 || v > 2*n {
			panic("bad number")
		}
		if q[v] {
			panic("number duplicate")
		}
		q[v] = true
	}
	for _, v := range t.p2 {
		if v < 0 || v > 2*n {
			panic("bad number")
		}
		if q[v] {
			panic("number duplicate")
		}
		q[v] = true
	}
}

func validatePartitioning() {
	var q [N]bool
	for _, t := range p {
		for _, v := range t.p1 {
			if v < 0 || v > 2*n {
				panic("bad number")
			}
			if q[v] {
				panic("number duplicate")
			}
			q[v] = true
		}
		for _, v := range t.p2 {
			if v < 0 || v > 2*n {
				panic("bad number")
			}
			if q[v] {
				panic("number duplicate")
			}
			q[v] = true
		}
	}
	for v := 1; v <= 2*n; v++ {
		if !q[v] {
			panic("missing number")
		}
	}
}
