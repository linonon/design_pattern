package builder

type BuilderType string

const (
	NormalBuilderType BuilderType = "normal"
	IglooBuilderType  BuilderType = "igloo"
)

type IBuilder interface {
	setWindowsType()
	setDoorType()
	setNumFloor()
	getHouse() House
}

func getBuilder(builderType BuilderType) IBuilder {
	switch builderType {
	case NormalBuilderType:
		return newNormalBuilder()
	case IglooBuilderType:
		return newIglooBuilder()
	default:
		return nil
	}
}
