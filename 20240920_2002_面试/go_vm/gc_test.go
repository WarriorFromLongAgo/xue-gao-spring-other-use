package go_vm

import (
	"fmt"
	"runtime"
	"testing"
)

func Test_go_vm(t *testing.T) {
	fmt.Println("Test_go_vm")

	// 打印GC信息
	printGCStats()

	// 创建大量临时对象
	for i := 0; i < 1000000; i++ {
		_ = make([]byte, 1024)
	}

	// 强制GC
	runtime.GC()

	// 打印GC信息
	printGCStats()
}

func printGCStats() {
	var stats runtime.MemStats
	runtime.ReadMemStats(&stats)
	fmt.Printf("Alloc = %v MiB", bToMb(stats.Alloc))
	fmt.Printf("\tTotalAlloc = %v MiB", bToMb(stats.TotalAlloc))
	fmt.Printf("\tSys = %v MiB", bToMb(stats.Sys))
	fmt.Printf("\tNumGC = %v\n", stats.NumGC)
}

func bToMb(b uint64) uint64 {
	return b / 1024 / 1024
}
