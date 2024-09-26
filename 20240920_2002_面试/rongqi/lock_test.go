package rongqi

import (
	"fmt"
	"sync"
	"testing"
)

type UserAges struct {
	ages map[string]int
	sync.Mutex
}

func (ua *UserAges) Add(name string, age int) {
	ua.Lock()
	defer ua.Unlock()
	ua.ages[name] = age
}

func (ua *UserAges) Get(name string) int {
	if age, ok := ua.ages[name]; ok {
		return age
	}
	return -1
}

func Test_lock_test_1(t *testing.T) {
	fmt.Println("Test_lock_test_1")

	ua := &UserAges{ages: make(map[string]int)}

	// 测试 Add 方法
	ua.Add("Alice", 30)
	if age := ua.Get("Alice"); age != 30 {
		t.Errorf("Expected age 30, got %d", age)
	}

	// 测试 Get 方法
	ua.Add("Bob", 25)
	if age := ua.Get("Bob"); age != 25 {
		t.Errorf("Expected age 25, got %d", age)
	}

	// 测试不存在的用户
	if age := ua.Get("Charlie"); age != -1 {
		t.Errorf("Expected age -1 for non-existent user, got %d", age)
	}

	// 启动多个 goroutine 同时调用 Add 和 Get 方法
	for i := 0; i < 100; i++ {
		go func(i int) {
			name := fmt.Sprintf("User%d", i)
			ua.Add(name, i)
			if age := ua.Get(name); age != i {
				t.Errorf("Expected age %d, got %d", i, age)
			}
		}(i)
	}
}
