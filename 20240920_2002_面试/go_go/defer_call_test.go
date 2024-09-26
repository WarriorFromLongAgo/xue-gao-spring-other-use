package go_go

import (
	"fmt"
	"testing"
)

func Test_defer_call(t *testing.T) {
	fmt.Println("Test_defer_call")

	deferCall()
	fmt.Println("=====================================")
	fmt.Println("=====================================")
	fmt.Println("=====================================")
	testDeferOrder()
}

func deferCall() {
	defer func() { fmt.Println("打印前") }()
	defer func() { fmt.Println("打印中") }()
	defer func() { fmt.Println("打印后") }()
}

func testDeferOrder() {
	fmt.Println("Start of function")

	defer fmt.Println("First defer")
	defer fmt.Println("Second defer")
	defer fmt.Println("Third defer")

	fmt.Println("End of function")
}
