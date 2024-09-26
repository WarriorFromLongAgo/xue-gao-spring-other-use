package 算法题

import (
	"fmt"
	"testing"
)

func Test_反转字符串(t *testing.T) {
	fmt.Println("Test_反转字符串")

	str, flag := revertStr("abcd")
	fmt.Println(flag)
	fmt.Println(str)
}

func revertStr(str string) (string, bool) {
	runeArr := []rune(str)
	l := len(runeArr)
	if l > 5000 {
		return str, false
	}
	for i := 0; i < l/2; i++ {
		runeArr[i], runeArr[l-1-i] = runeArr[l-1-i], runeArr[i]
	}
	return string(runeArr), true
}
