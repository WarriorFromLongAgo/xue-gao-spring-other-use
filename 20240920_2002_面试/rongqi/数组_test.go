package rongqi

import (
	"fmt"
	"testing"
)

func Test_make_init(t *testing.T) {
	fmt.Println("Test_make_init")

	s := make([]int, 5)
	s = append(s, 1, 2, 3)
	fmt.Println(s)

	// [0 0 0 0 0 1 2 3]
	// make 在初始化切片时指定了长度，所以追加数据时会从len(s) 位置开始填充数据。
}
