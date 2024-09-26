package go_go

import (
	"fmt"
	"testing"
)

type People interface {
	Speak(string) string
}

type Student struct{}

func (stu *Student) Speak(think string) (talk string) {
	if think == "bitch" {
		talk = "You are a good boy"
	} else {
		talk = "hi"
	}
	return
}

func Test_impl(t *testing.T) {
	var peo People = &Student{}
	think := "bitch"
	fmt.Println(peo.Speak(think))
}

func live() People {
	var stu *Student
	return stu
}

func Test_impl_v2(t *testing.T) {
	if live() == nil {
		fmt.Println("AAAAAAA")
	} else {
		fmt.Println("BBBBBBB")
	}
}
