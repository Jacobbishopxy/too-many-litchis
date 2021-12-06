package problems

func Generate_118(numRows int) [][]int {

	result := make([][]int, numRows)

	for n := range result {
		result[n] = make([]int, n+1)
		result[n][0] = 1
		result[n][n] = 1
		for i := 1; i < n; i++ {
			result[n][i] = result[n-1][i-1] + result[n-1][i]
		}
	}

	return result
}
