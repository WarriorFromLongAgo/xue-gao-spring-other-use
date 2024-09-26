package goroutine_channel_sync

import (
	"fmt"
	"sync"
	"testing"
)

func Test_20_goroutine(t *testing.T) {
	fmt.Println("Test_20_goroutine")

	//runtime.GOMAXPROCS(1)
	wg := sync.WaitGroup{}
	wg.Add(20)
	for i := 0; i < 10; i++ {
		go func() {
			fmt.Println("i: ", i)
			wg.Done()
		}()
	}
	for i := 0; i < 10; i++ {
		go func(i int) {
			fmt.Println("j: ", i)
			wg.Done()
		}(i)
	}
	wg.Wait()
}
