package main

import (
	"fmt"
	"os"
	"strconv"
)

func main() {
	args := os.Args[1:]
	if len(args) < 1 {
		fmt.Println("Challenge day is missing.")
		return
	}

	challengeDay, err := strconv.ParseUint(args[0], 10, 5)
	if err != nil {
		fmt.Printf("Day can only be 19–25: %v\n", err)
		return
	}

	fmt.Println("Day:", challengeDay)

	switch challengeDay {
	case 19:
		run19(args[1:])
	// case 20:
	// 	run20()
	// case 21:
	// 	run21()
	// case 22:
	// 	run22()
	// case 23:
	// 	run23()
	// case 24:
	// 	run24()
	// case 25:
	// 	run25()
	default:
		fmt.Println("Challenge day is out of range.")
	}
}
