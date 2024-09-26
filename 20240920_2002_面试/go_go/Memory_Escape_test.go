package go_go

import (
	"fmt"
	"testing"
)

type A struct {
	s string
}

// 这是上面提到的 "在方法内把局部变量指针返回" 的情况
func foo(s string) *A {
	a := new(A)
	a.s = s
	return a //返回局部变量a,在C语言中妥妥野指针，但在go则ok，但a会逃逸到堆
}

func Test_Memory_Escape(t *testing.T) {
	a := foo("hello")
	b := a.s + " world"
	c := b + "!"
	fmt.Println(c)
}
