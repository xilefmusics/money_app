package data

import (
	"fmt"
	"reflect"
	"testing"
)

func TestGetListOfUsers(t *testing.T) {
	users, err := getListOfUsers("./testData/transactions")
	if err != nil {
		t.Fatalf("ERROR: getListOfUsers failed with an error (%s)", err.Error())
	}
	if !reflect.DeepEqual(users, []string{"1", "2"}) {
		fmt.Println(users)
		t.Fatalf("ERROR: getLIstOfUsers produced a wrong output\n")
	}
}

func TestNew(t *testing.T) {
	data, err := New("./testData")
	if err != nil {
		t.Fatalf("ERROR: New failed with an error (%s)", err.Error())
	}
	fmt.Println(data)
}
