package abstractfactory

type Nike struct{}

func (n *Nike) makeShoe() IShoe {
	return &NikeShoe{
		Shoe: Shoe{
			logo: "nike",
			size: 15,
		},
	}
}

func (n *Nike) makeShirt() IShirt {
	return &NikeShirt{
		Shoe: Shoe{
			logo: "nike",
			size: 15,
		},
	}
}
