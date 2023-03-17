package helper

func TestUniqueMergeStringSlices(t *testing.T) {
	s1 := []string{"1", "2"}
	s2 := []string{"2", "3"}
	sR := []string{"1", "2", "3"}
	merged := uniqueMergeStringSlices(s1, s2)
	if !reflect.DeepEqual(sR, merged) {
		fmt.Println(sR)
		fmt.Println(merged)
		t.Fatalf("ERROR: uniqueMergeStringSlices produced a wrong output\n")
	}
}
