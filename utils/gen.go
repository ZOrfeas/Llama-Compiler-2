package utils

type GenIterator[Item any] <-chan Item

/////////////////////////////////
// General generator utilities //
/////////////////////////////////

type Generator[Item any] interface {
	Next() (Item, bool)
	Iter() GenIterator[Item]
}

func GenMap[Item any, ItemOut any](gen Generator[Item], fn func(Item) ItemOut) Generator[ItemOut] {
	itemOutChan := make(chan ItemOut)
	go func() {
		defer close(itemOutChan)
		for item := range gen.Iter() {
			itemOutChan <- fn(item)
		}
	}()
	return &basicGenerator[ItemOut]{itemOutChan}
}
func GenFilter[Item any](gen Generator[Item], fn func(Item) bool) Generator[Item] {
	itemChan := make(chan Item)
	go func() {
		defer close(itemChan)
		for item := range gen.Iter() {
			if fn(item) {
				itemChan <- item
			}
		}
	}()
	return &basicGenerator[Item]{itemChan}
}
func GenIntoPeekable[Item any](gen Generator[Item]) PeekableGenerator[Item] {
	var peekedItem Item
	return &basicPeekableGenerator[Item]{gen, peekedItem, false}
}
func GenUnroll[Item any](gen Generator[Item]) {
	for range gen.Iter() {
	}
}

////////////////////////
// Peekable Generator //
////////////////////////

type PeekableGenerator[Item any] interface {
	Generator[Item]
	Peek() (Item, bool)
}
type basicPeekableGenerator[Item any] struct {
	innerGen   Generator[Item]
	peekedItem Item
	peeked     bool
}

// Returns the peeked value if it exists, otherwise iterates over the inner generator once.
func (gen *basicPeekableGenerator[Item]) Next() (Item, bool) {
	if gen.peeked {
		gen.peeked = false
		return gen.peekedItem, true
	} else {
		return gen.innerGen.Next()
	}
}

// Ignores peeked value if exists and just iterates over the inner generator.
func (gen *basicPeekableGenerator[Item]) Iter() GenIterator[Item] {
	return gen.innerGen.Iter()
}

// Returns the peeked value if it exists, otherwise peeks one.
func (gen basicPeekableGenerator[Item]) Peek() (Item, bool) {
	if !gen.peeked {
		gen.peekedItem, gen.peeked = gen.Next()
	}
	return gen.peekedItem, gen.peeked
}

//////////////////////////////////////////////////////////////////////////////
// Basic Generator implementation for compliance to the generator interface //
//////////////////////////////////////////////////////////////////////////////

type basicGenerator[Item any] struct {
	itemChan chan Item
}

func (gen *basicGenerator[Item]) Next() (Item, bool) {
	item, ok := <-gen.itemChan
	return item, ok
}
func (gen *basicGenerator[Item]) Iter() GenIterator[Item] {
	return gen.itemChan
}
