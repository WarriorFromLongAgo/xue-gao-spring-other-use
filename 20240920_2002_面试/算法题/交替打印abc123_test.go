package 算法题

import (
	"fmt"
	"sync"
	"testing"
)

func Test_交替打印abc123(t *testing.T) {
	fmt.Println("Test_交替打印abc123 Test_交替打印abc123 ")
	numChan := make(chan bool)
	letterChan := make(chan bool)

	var wg sync.WaitGroup

	wg.Add(2)

	// 打印数字的 goroutine
	go func() {
		defer wg.Done()

		for i := 0; i < 26; i++ {
			<-numChan
			fmt.Print(i)
			letterChan <- true
		}
	}()

	// 打印字母的 goroutine
	go func() {
		defer wg.Done()

		for i := 0; i < 26; i++ {
			<-letterChan
			fmt.Printf("%c", 'A'+i)
			if i < 25 {
				numChan <- true
			}
		}
	}()

	// 启动打印
	numChan <- true

	wg.Wait()
	//close(numChan)
	//close(letterChan)
}

func Test_交替打印abc123_v2(t *testing.T) {
	fmt.Println("Test_交替打印abc123 Test_交替打印abc123 ")
	numChan := make(chan bool)
	letterChan := make(chan bool)

	var wg sync.WaitGroup

	wg.Add(2)

	// 打印数字的 goroutine
	go func() {
		defer wg.Done()

		index := 0
		for {
			select {
			case <-numChan:
				if index >= 26 {
					close(letterChan)
					return
				}
				fmt.Print(index)
				index++
				fmt.Print(index)
				index++
				letterChan <- true
			}
		}
	}()

	// 打印字母的 goroutine
	go func() {
		defer wg.Done()

		index := 'A'
		for {
			select {
			case <-letterChan:
				if index >= 'Z' {
					close(numChan)
					return
				}
				fmt.Printf("%c", index)
				index++
				fmt.Printf("%c", index)
				index++
				numChan <- true
			}
		}
	}()

	// 启动打印
	numChan <- true

	wg.Wait()
	//close(numChan)
	//close(letterChan)
}

func Test_交替打印abc123_v3(t *testing.T) {
	fmt.Println("Test_交替打印abc123 Test_交替打印abc123 ")
	numChan := make(chan bool)
	letterChan := make(chan bool)
	done := make(chan bool)

	// 打印数字的 goroutine
	go func() {
		index := 0
		for {
			select {
			case <-numChan:
				if index >= 26 {
					close(letterChan)
					return
				}
				fmt.Print(index)
				index++
				fmt.Print(index)
				index++
				letterChan <- true
			}
		}
	}()

	// 打印字母的 goroutine
	go func() {
		index := 'A'
		for {
			select {
			case <-letterChan:
				if index >= 'Z' {
					done <- true
					close(numChan)
					return
				}
				fmt.Printf("%c", index)
				index++
				fmt.Printf("%c", index)
				index++
				numChan <- true
			}
		}
	}()

	// 启动打印
	numChan <- true

	<-done
}
