package builder

type NormalBuilder struct {
	windowType string
	doorType   string
	floor      int
}

func newNormalBuilder() *NormalBuilder {
	return &NormalBuilder{}
}

func (b *NormalBuilder) setWindowsType() {
	b.windowType = "Snow Window"
}

func (b *NormalBuilder) setDoorType() {
	b.doorType = "Snow Door"
}

func (b *NormalBuilder) setNumFloor() {
	b.floor = 1
}

func (b *NormalBuilder) getHouse() House {
	return House{
		windowType: b.windowType,
		doorType:   b.doorType,
		floor:      b.floor,
	}
}
