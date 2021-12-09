package problems

import (
	"fmt"
	"testing"
	"tml/util"
)

func Test124(t *testing.T) {
	root := &util.TreeNode{Val: -10}
	root.Left = &util.TreeNode{Val: 9}
	root.Right = &util.TreeNode{Val: 20}
	root.Right.Left = &util.TreeNode{Val: 15}
	root.Right.Right = &util.TreeNode{Val: 7}

	fmt.Printf("%v\n", MaxPathSum_124(root))
}
