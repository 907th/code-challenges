func isArraySpecial(nums []int) bool {
	for i := 1; i < len(nums); i++ {
		if nums[i]%2 == nums[i-1]%2 {
			return false
		}
	}
	return true
}
