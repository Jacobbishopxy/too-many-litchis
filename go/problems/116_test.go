package problems

import (
	"fmt"
	"testing"

	util "tml/util"
)

/*
go test -v 116_test.go
*/
func TestFoo(t *testing.T) {

	node := util.NewNodeFromVals([]int{1, 2, 3, 4, 5, 6, 7})

	n := Connect(node)

	fmt.Printf("%+v", n.Val)
}
