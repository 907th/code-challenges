func divides(s, t string) bool {
	if len(s)%len(t) != 0 {
		return false
	}
	for i := 0; i < len(s); i += len(t) {
		if s[i:i+len(t)] != t {
			return false
		}
	}
	return true
}

func gcdOfStrings(str1 string, str2 string) string {
	var res string
	for l := 1; l <= min(len(str1), len(str2)); l++ {
		t := str1[:l]
		if divides(str1, t) && divides(str2, t) {
			res = t
		}
	}
	return res
}
