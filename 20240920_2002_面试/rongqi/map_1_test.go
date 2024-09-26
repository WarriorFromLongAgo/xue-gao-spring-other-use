package rongqi

import (
	"fmt"
	"testing"
)

type Student struct {
	Age int
}

func Test_map_1(t *testing.T) {
	kv := map[string]*Student{"menglu": {Age: 21}}
	kv["menglu"].Age = 22
	s := []Student{{Age: 21}}
	s[0].Age = 22
	fmt.Println(kv, s)
}
