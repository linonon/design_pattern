package strategy

type EvictionAlgo interface {
	// evict 應該要將數據刪除掉
	evict(c *Cache)
}
