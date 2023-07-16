package abstractfactory

import (
	"fmt"
	"testing"
)

func TestAbstractFactory(t *testing.T) {
	adidasFactory, _ := GetSportFactory("adidas")
	nikeFactor, _ := GetSportFactory("nike")

	nikeShoe := nikeFactor.makeShoe()
	nikeShirt := nikeFactor.makeShirt()

	adidasShoe := adidasFactory.makeShoe()
	adidasShirt := adidasFactory.makeShirt()

	printShoeDetails(nikeShoe)
	printShirtDetails(nikeShirt)

	printShoeDetails(adidasShoe)
	printShirtDetails(adidasShirt)
}

func printShoeDetails(s IShoe) {
	fmt.Printf("Shoe's Logo: %s\n", s.getLogo())
	fmt.Printf("Shoe's Size: %d\n", s.getSize())
}

func printShirtDetails(s IShirt) {
	fmt.Printf("Shirt's Logo: %s\n", s.getLogo())
	fmt.Printf("Shirt's Size: %d\n", s.getSize())
}
