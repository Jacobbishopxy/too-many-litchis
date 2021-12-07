package problems

import (
	"fmt"
	"testing"
)

func Test122(t *testing.T) {
	var foo []int

	foo = []int{7, 1, 5, 3, 6, 4}
	fmt.Printf("%v\n", MaxProfit_122(foo))

	foo = []int{1, 2, 3, 4, 5}
	fmt.Printf("%v\n", MaxProfit_122(foo))

	foo = []int{7, 6, 4, 3, 1}
	fmt.Printf("%v\n", MaxProfit_122(foo))

	foo = []int{2, 1, 2, 0, 1}
	fmt.Printf("%v\n", MaxProfit_122(foo))
}
