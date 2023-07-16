package abstractfactory

import "fmt"

type ISportFactory interface {
	makeShoe() IShoe
	makeShirt() IShirt
}

// GetSportFactory 提供兩個工廠函數的方法
//
//	"adidas", "nike"
func GetSportFactory(brand string) (ISportFactory, error) {
	if brand == "adidas" {
		return &Adidas{}, nil
	}
	if brand == "nike" {
		return &Nike{}, nil
	}
	return nil, fmt.Errorf("wrong  brand type passed")
}
