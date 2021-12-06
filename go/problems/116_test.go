package problems

import (
	"fmt"
	"testing"

	util "tml/util"
)

/*
go test -v 116_test.go
*/
func Test116(t *testing.T) {

	node := util.NewNodeFromVals([]int{1, 2, 3, 4, 5, 6, 7})

	n := Connect_116(node)

	fmt.Printf("%+v", n.Val)
}
