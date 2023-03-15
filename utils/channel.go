package utils

type Generator[Item any] interface {
	Next() (Item, bool)
	Iter() <-chan Item
}

type BasicGenerator[Item any] struct {
	itemChan chan Item
}

func (gen *BasicGenerator[Item]) Next() (Item, bool) {
	item, ok := <-gen.itemChan
	return item, ok
}
func (gen *BasicGenerator[Item]) Iter() <-chan Item {
	return gen.itemChan
}

func GenMap[Item any, ItemOut any](gen Generator[Item], fn func(Item) ItemOut) Generator[ItemOut] {
	itemOutChan := make(chan ItemOut)
	go func() {
		defer close(itemOutChan)
		for item := range gen.Iter() {
			itemOutChan <- fn(item)
		}
	}()
	return &BasicGenerator[ItemOut]{itemOutChan}
}

func GenUnroll[Item any](gen Generator[Item]) {
	for range gen.Iter() {
	}
}
