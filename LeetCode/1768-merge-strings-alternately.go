func mergeAlternately(word1 string, word2 string) string {
	var i1, i2 int
	res := make([]byte, len(word1)+len(word2))
	for i1 < len(word1) || i2 < len(word2) {
		if i1 < len(word1) {
			res[i1+i2] = word1[i1]
			i1++
		}
		if i2 < len(word2) {
			res[i1+i2] = word2[i2]
			i2++
		}
	}
	return string(res)
}
