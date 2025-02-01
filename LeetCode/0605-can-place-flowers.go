func canPlaceFlowers(flowerbed []int, m int) bool {
	n := len(flowerbed)
	k := 0
	l := 0
	for r := 0; r <= n; r++ {
		if r == n || flowerbed[r] == 1 {
			x := r - l
			if l > 0 {
				x -= 1
			}
			if r < n {
				x -= 1
			}
			if x > 0 {
				k += (x + 1) / 2
			}
			l = r + 1
		}
	}
	return k >= m
}
