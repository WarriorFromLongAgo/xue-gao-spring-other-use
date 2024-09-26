package go_go

import (
	"fmt"
	"testing"
)

func Test_for_range(t *testing.T) {
	fmt.Println("Test_for_range")

	parseStudent()
	fmt.Println("=================================")
	fmt.Println("=================================")
	fmt.Println("=================================")
	parseStudentV2()
}

type student struct {
	Name string
	Age  int
}

func parseStudent() {
	m := make(map[string]*student)
	stus := []student{
		{Name: "zhou", Age: 24},
		{Name: "li", Age: 23},
		{Name: "wang", Age: 22},
	}
	for _, stu := range stus {
		m[stu.Name] = &stu
	}

	fmt.Println("stus", stus)
	for k, v := range m {
		fmt.Printf("key: %s, value: %+v\n", k, *v)
	}
}

func parseStudentV2() {
	m := make(map[string]*student)
	stus := []student{
		{Name: "zhou", Age: 24},
		{Name: "li", Age: 23},
		{Name: "wang", Age: 22},
	}
	for _, stu := range stus {
		m[stu.Name] = &stu
	}

	fmt.Println("stus", stus)
	for k, v := range m {
		fmt.Printf("key: %s, value: %+v\n", k, *v)
	}
}
