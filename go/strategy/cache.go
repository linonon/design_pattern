package strategy

import "fmt"

type Cache struct {
	storage      map[string]string
	evictionAlgo EvictionAlgo
	capacity     int
	maxCapacity  int
}

// initCache 初始化, 但是需要有一個基礎的 Algo
func initCache(e EvictionAlgo) *Cache {
	storage := make(map[string]string)
	return &Cache{
		storage:      storage,
		evictionAlgo: e,
		capacity:     0,
		maxCapacity:  2,
	}
}

func (c *Cache) setEvictionAlgo(e EvictionAlgo) {
	c.evictionAlgo = e
}

func (c *Cache) evict() {
	c.evictionAlgo.evict(c)
	c.capacity--
}

// delete 將數據從 Cache 中取出
func (c *Cache) delete(key string) {
	delete(c.storage, key)
}

func (c *Cache) add(key, value string) {
	if c.capacity == c.maxCapacity {
		c.evict()
	}
	c.capacity++
	c.storage[key] = value
}

func (c *Cache) NowCapacity() int {
	return c.capacity
}

func (c *Cache) ShowData() string {
	return fmt.Sprintf("%+v\n", c.storage)
}
