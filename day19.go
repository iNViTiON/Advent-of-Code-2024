package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

var cache = make(map[string]uint)

func isPossible(patterns []string, strip string) uint {
	if len(strip) == 0 {
		return uint(1)
	}
	if val, ok := cache[strip]; ok {
		return val
	}
	count := uint(0)
	for _, pattern := range patterns {
		if strings.HasPrefix(strip, pattern) {
			count += isPossible(patterns, strip[len(pattern):])
		}
	}
	cache[strip] = count
	return count
}

func run19(args []string) {
	if len(args) < 1 {
		fmt.Println("Input file is missing")
	}

	inputFilePath := args[0]
	fmt.Println("Input file:", inputFilePath)

	inputFile, err := os.Open(inputFilePath)
	if err != nil {
		fmt.Println("Error reading file:", err)
	}
	defer inputFile.Close()

	inputFileScanner := bufio.NewScanner(inputFile)

	var firstLine string
	if inputFileScanner.Scan() {
		firstLine = inputFileScanner.Text()
	}
	patterns := strings.Split(firstLine, ", ")

	p := uint(0)
	p2 := uint(0)
	for inputFileScanner.Scan() {
		strip := inputFileScanner.Text()
		if len(strip) == 0 {
			continue
		}
		patternPossibleCount := isPossible(patterns, strip)
		if patternPossibleCount > 0 {
			p++
		}
		p2 += patternPossibleCount
	}
	fmt.Println("Possible patterns:", p)
	fmt.Println("Summarize of way to arrange patterns count:", p2)
}
