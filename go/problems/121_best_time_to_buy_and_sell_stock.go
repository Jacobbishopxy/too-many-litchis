package problems

import "math"

func MaxProfit_121(prices []int) int {
	maxP := 0
	minP := math.MaxInt64

	for start := 0; start < len(prices); start++ {
		if prices[start] < minP {
			minP = prices[start]
		} else if prices[start]-minP > maxP {
			maxP = prices[start] - minP
		}
	}
	return maxP
}
