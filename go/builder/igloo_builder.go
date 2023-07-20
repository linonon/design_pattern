package builder

type IglooBuilder struct {
	windowType string
	doorType   string
	floor      int
}

func newIglooBuilder() *IglooBuilder {
	return &IglooBuilder{}
}

func (b *IglooBuilder) setWindowsType() {
	b.windowType = "Classic Window"
}

func (b *IglooBuilder) setDoorType() {
	b.doorType = "Classic Door"
}

func (b *IglooBuilder) setNumFloor() {
	b.floor = 3
}

func (b *IglooBuilder) getHouse() House {
	return House{
		windowType: b.windowType,
		doorType:   b.doorType,
		floor:      b.floor,
	}
}
