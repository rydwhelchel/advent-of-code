package helpers

import (
	"log"
	"os"
	"strings"
	"time"
)

type Day interface {
	Name() string
	Part1() string
	Part2() string
}

func Solve(day Day) {
	t := time.Now()
	log.Printf("~~~~~~~~~~ %v ~~~~~~~~~~", day.Name())
	log.Printf("Part 1: %v\n", day.Part1())
	log.Printf("	in %.6f seconds\n", time.Now().Sub(t).Seconds())
	t = time.Now()
	log.Printf("Part 2: %v\n", day.Part2())
	log.Printf("	in %.6f seconds\n", time.Now().Sub(t).Seconds())
}

//	func ReadInputAsString(name string) string {
//		return ReadXAsString(name + "/input")
//	}

func ReadXAsString(name string) string {
	inputFile := name
	data, err := os.ReadFile(inputFile)
	if err != nil {
		log.Panicln("Failed to read file")
	}
	return string(data)
}

// ReadInputAsLines does not include the new line characters
func ReadInputAsLines(day string) []string {
	return ReadXAsLines(day + "/input")
}

// ReadXAsLines does not include the new line characters
func ReadXAsLines(name string) []string {
	data := ReadXAsString(name)
	return strings.Split(data, "\n")
}
