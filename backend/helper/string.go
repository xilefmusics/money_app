package helper

func StringUniqueMergeSlices(slice1 []string, slice2 []string) []string {
	result := slice1[:]
	for _, s2 := range slice2 {
		found := false
		for _, r := range result {
			if r == s2 {
				found = true
				break
			}
		}
		if !found {
			result = append(result, s2)
		}
	}
	return result
}
