package problems

import "tml/util"

func Connect_116(root *util.Node) *util.Node {
	if root == nil {
		return root
	}
	for lm := root; lm.Left != nil; lm = lm.Left {
		for node := lm; node != nil; node = node.Next {
			node.Left.Next = node.Right

			if node.Next != nil {
				node.Right.Next = node.Next.Left
			}
		}
	}
	return root
}
