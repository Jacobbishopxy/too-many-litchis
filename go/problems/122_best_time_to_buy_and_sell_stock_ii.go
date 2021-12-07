package problems

func MaxProfit_122(prices []int) int {
	var res int
	for i := 1; i < len(prices); i++ {
		res += max(0, prices[i]-prices[i-1])
	}
	return res
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}
