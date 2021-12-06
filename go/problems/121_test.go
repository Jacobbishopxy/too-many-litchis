package problems

import (
	"fmt"
	"testing"
)

func Test121(t *testing.T) {
	foo := []int{7, 1, 5, 3, 6, 4}

	fmt.Printf("%v\n", MaxProfit_121(foo))

	bar := []int{7, 6, 4, 3, 1}

	fmt.Printf("%v\n", MaxProfit_121(bar))

	qux := []int{2, 1, 4}

	fmt.Printf("%v\n", MaxProfit_121(qux))
}
