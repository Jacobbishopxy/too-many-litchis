package util

type Node struct {
	Val   int
	Left  *Node
	Right *Node
	Next  *Node
}

func NewNode(val int) *Node {
	return &Node{Val: val}
}

func NewNodeFromVals(vals []int) *Node {
	n := len(vals)
	if n == 0 {
		return nil
	}

	root := &Node{Val: vals[0]}

	queue := make([]*Node, 1, n*2)
	queue[0] = root

	i := 1
	for i < n {
		node := queue[0]
		queue = queue[1:]

		if i < n && vals[i] != 0 {
			node.Left = &Node{Val: vals[i]}
			queue = append(queue, node.Left)
		}
		i++

		if i < n && vals[i] != 0 {
			node.Right = &Node{Val: vals[i]}
			queue = append(queue, node.Right)
		}
		i++
	}

	return root
}
