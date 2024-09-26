package goroutine_channel_sync

import (
	"fmt"
	"math/rand"
	"runtime"
	"testing"
)

func Test_channel_select_test(t *testing.T) {
	fmt.Println("Test_channel_select_test")

	runtime.GOMAXPROCS(1)
	intChan := make(chan int, 1)
	stringChan := make(chan string, 1)
	intChan <- 1
	stringChan <- "hello"
	select {
	case value := <-intChan:
		fmt.Println(value)
	case value := <-stringChan:
		panic(value)
	}
}

func Test_channel_select_test_v2(t *testing.T) {
	random := make(chan int)
	done := make(chan bool)

	go func() {
		defer close(random)
		for i := 0; i < 5; i++ {
			random <- rand.Int()
		}
	}()

	go func() {
		for {
			select {
			case num, ok := <-random:
				if ok {
					fmt.Println(num)
					continue
				}
				done <- true
			}
		}
	}()

	<-done
}
