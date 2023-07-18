package strategy

import (
	"fmt"
	"testing"
)

func TestEviction(t *testing.T) {
	lfu := &Lfu{}
	cache := initCache(lfu)

	cache.add("a", "1")
	fmt.Println(cache.NowCapacity())
	cache.add("b", "2")
	cache.add("c", "3")
	fmt.Println(cache.NowCapacity())

	lru := &Lru{}
	cache.setEvictionAlgo(lru)

	cache.add("d", "4")
	fmt.Println(cache.NowCapacity())

	fifo := &Fifo{}
	cache.setEvictionAlgo(fifo)

	cache.add("e", "5")
	fmt.Println(cache.NowCapacity())

	cache.delete("a")
	fmt.Println(cache.NowCapacity())
	fmt.Println(cache.ShowData())
}
