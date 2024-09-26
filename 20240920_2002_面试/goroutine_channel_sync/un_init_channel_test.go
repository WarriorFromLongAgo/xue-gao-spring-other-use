package goroutine_channel_sync

import (
	"fmt"
	"testing"
	"time"
)

func Test_un_init_channel_test(t *testing.T) {
	fmt.Println("Test_un_init_channel_test")

	var ch chan int // 未初始化的channel

	// 尝试从未初始化的channel读取数据
	go func() {
		fmt.Println("Attempting to read from nil channel...")
		val := <-ch
		fmt.Println("Read value:", val)
	}()

	// 尝试向未初始化的channel写入数据
	go func() {
		fmt.Println("Attempting to write to nil channel...")
		ch <- 42
		fmt.Println("Wrote value to channel")
	}()

	// 等待一段时间以观察结果
	time.Sleep(2 * time.Second)
	fmt.Println("Main function completed")
}
