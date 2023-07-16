package factory

import (
	"fmt"
	"testing"
)

type IGun interface {
	setName(name string)
	setPower(power int)
	getName() string
	getPower() int
}

type Gun struct {
	name  string
	power int
}

func (g *Gun) setName(name string) {
	g.name = name
}

func (g *Gun) setPower(power int) {
	g.power = power
}

func (g *Gun) getName() string {
	return g.name
}

func (g *Gun) getPower() int {
	return g.power
}

type Ak47 struct {
	Gun
}

func newAk47() IGun {
	return &Ak47{
		Gun: Gun{
			name:  "ak47 gun",
			power: 4,
		},
	}
}

type Musket struct {
	Gun
}

func newMusket() IGun {
	return &Ak47{
		Gun: Gun{
			name:  "Musket gun",
			power: 1,
		},
	}
}

func getGun(gunName string) (IGun, error) {
	if gunName == "ak47" {
		return newAk47(), nil
	}
	if gunName == "musket" {
		return newMusket(), nil
	}

	return nil, fmt.Errorf("wrong gun type pass")
}

func TestFactoryIGun(t *testing.T) {
	ak47, _ := getGun("ak47")
	musket, _ := getGun("musket")

	printDetails(ak47)
	printDetails(musket)
}

func printDetails(g IGun) {
	fmt.Printf("Gun: %s\n", g.getName())
	fmt.Printf("Power: %d\n", g.getPower())
}
