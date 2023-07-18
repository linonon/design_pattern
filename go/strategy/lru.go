package strategy

import "fmt"

type Lru struct{}

func (l *Lru) evict(c *Cache) {
	fmt.Println("Evciting by Lru Algo")
}
