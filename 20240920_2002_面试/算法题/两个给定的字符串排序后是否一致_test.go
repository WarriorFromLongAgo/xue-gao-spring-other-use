package 算法题

import (
	"fmt"
	"strings"
	"testing"
)

func Test_两个给定的字符串排序后是否一致(t *testing.T) {
	fmt.Println("Test_反转字符串")

	flag := strIsEqual("abcd", "abdc")
	fmt.Println(flag)
}

func strIsEqual(str1 string, str2 string) bool {
	runeArr1 := []rune(str1)

	for _, value := range runeArr1 {
		if strings.Count(str1, string(value)) != strings.Count(str2, string(value)) {
			return false
		}
	}
	return true
}
