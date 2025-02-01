func kidsWithCandies(candies []int, extraCandies int) []bool {
	n := len(candies)
	r := make([]bool, n)
	for i := 0; i < n; i++ {
		cur := candies[i] + extraCandies
		r[i] = true
		for j := 0; j < n; j++ {
			if j != i && candies[j] > cur {
				r[i] = false
				break
			}
		}
	}
	return r
}
