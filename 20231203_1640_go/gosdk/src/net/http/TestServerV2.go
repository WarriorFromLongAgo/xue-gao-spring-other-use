package main

import (
	"fmt"
	"net/http"
)

type HelloHandlerStruct struct {
	content string
}

func (handler *HelloHandlerStruct) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	fprintf, err := fmt.Fprintf(w, handler.content)
	if err != nil {
		return
	}
	fmt.Println(fprintf)
}

func main() {
	http.Handle("/", &HelloHandlerStruct{content: "Hello World"})
	err := http.ListenAndServe(":8000", nil)
	if err != nil {
		return
	}
}
