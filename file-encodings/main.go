package main

import (
	"fmt"
	"unicode/utf8"
)

func main() {
	r := rune('✊')
	fmt.Println("RUNE")
	fmt.Printf("Unicode: %#U\n", r)
	fmt.Printf("Binary representation: % 08b\n", r)
	fmt.Printf("Hex representation: %x\n", r)

	b := []byte("✊")
	fmt.Println("BYTES")
	fmt.Printf("Unicode: %#U\n", b)
	fmt.Printf("Binary representation: % 08b\n", b)
	fmt.Printf("Hex representation: %x\n", b)

	s := "С"
	b = []byte(s)

	for len(b) > 0 {
		r, size := utf8.DecodeRune(b)
		fmt.Printf("%#U %v bytes", r, size)
		fmt.Printf("% 08b", r)
		b = b[size:]
	}
}
