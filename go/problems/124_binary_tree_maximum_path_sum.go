package problems

import (
	"math"
	"tml/util"
)

func MaxPathSum_124(root *util.TreeNode) int {
	var num = math.MinInt32
	max_gain(&num, root)
	return num
}

func max_gain(num *int, node *util.TreeNode) int {
	if node == nil {
		return 0
	}

	left := max_gain(num, node.Left)
	right := max_gain(num, node.Right)

	if left < 0 {
		left = 0
	}

	if right < 0 {
		right = 0
	}

	*num = max(*num, left+right+node.Val)

	return max(left, right) + node.Val
}
