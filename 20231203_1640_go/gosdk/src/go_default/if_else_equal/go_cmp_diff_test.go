package if_else_equal

import (
	"fmt"
	"github.com/google/go-cmp/cmp"
	"testing"
)

func Test_diff(t *testing.T) {
	c1 := &Contact{Phone: "123456789", Email: "dj@example.com"}
	c2 := &Contact{Phone: "123456879", Email: "dj2@example.com"}
	u1 := User{Name: "dj", Age: 18, Contact: c1}
	u2 := User{Name: "dj2", Age: 18, Contact: c2}

	fmt.Println(cmp.Diff(u1, u2))
}
