package builder

import (
	"fmt"
	"testing"
)

func TestBuilder(t *testing.T) {
	fmt.Println("ok")
	nb := getBuilder(NormalBuilderType)
	ib := getBuilder(IglooBuilderType)

	director := newDirector(nb)
	normalHouse := director.buildHouse()

	fmt.Printf("nb door type: %s\n", normalHouse.doorType)
	fmt.Printf("nb window type: %s\n", normalHouse.windowType)
	fmt.Printf("nb floor number: %d\n", normalHouse.floor)

	director.setBuilder(ib)
	iglooHouse := director.buildHouse()
	fmt.Printf("ib door type: %s\n", iglooHouse.doorType)
	fmt.Printf("ib window type: %s\n", iglooHouse.windowType)
	fmt.Printf("ib floor number: %d\n", iglooHouse.floor)
}
