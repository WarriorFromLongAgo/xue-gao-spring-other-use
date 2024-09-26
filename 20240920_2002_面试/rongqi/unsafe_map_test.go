package rongqi

import (
	"fmt"
	"sync"
	"testing"
)

func Test_unsafe_map(t *testing.T) {
	// 创建一个线程不安全的map
	unsafeMap := make(map[int]int)

	// 使用WaitGroup等待所有goroutine完成
	var wg sync.WaitGroup
	wg.Add(2)

	// 启动第一个goroutine，向map中写入数据
	go func() {
		defer wg.Done()
		for i := 0; i < 1000; i++ {
			unsafeMap[i] = i
		}
	}()

	// 启动第二个goroutine，从map中读取数据
	go func() {
		defer wg.Done()
		for i := 0; i < 1000; i++ {
			_ = unsafeMap[i]
		}
	}()

	// 等待所有goroutine完成
	wg.Wait()

	fmt.Println("Finished")
}

func Test_unsafe_map_fix(t *testing.T) {
	// 创建一个线程不安全的map
	unsafeMap := make(map[int]int)
	var mu sync.Mutex

	// 使用WaitGroup等待所有goroutine完成
	var wg sync.WaitGroup
	wg.Add(2)

	// 启动第一个goroutine，向map中写入数据
	go func() {
		defer wg.Done()
		for i := 0; i < 1000; i++ {
			mu.Lock()
			unsafeMap[i] = i
			mu.Unlock()
		}
	}()

	// 启动第二个goroutine，从map中读取数据
	go func() {
		defer wg.Done()
		for i := 0; i < 1000; i++ {
			mu.Lock()
			_ = unsafeMap[i]
			mu.Unlock()
		}
	}()

	// 等待所有goroutine完成
	wg.Wait()

	fmt.Println("Finished")
}
