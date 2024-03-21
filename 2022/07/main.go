package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func main() {
    file, err := os.Open("input")
    if err != nil {
        log.Println("Error opening file")
        return
    }

    sc := bufio.NewScanner(file)
    lines := []string{}
    for sc.Scan() {
        text := sc.Text()
        lines = append(lines, text)
    }

    part1 := part1(lines)
    fmt.Printf("Part1: %v\n", part1)
}

// Files are counted as a directory with no subdirectories
type Directory struct {
    // Name -> Directory
    subdirectories map[string]Directory
    size int
    // Slice of 1 or none (in the case of /)
    parent []*Directory
}

func part1(lines []string) int {
    //1. Go through lines and construct Directory hierarchy, directories initially have size 0
    // Start in root
    curr := Directory {
        subdirectories: make(map[string]Directory),
        size: 0,
        parent: nil,
    }
    index := 0
    for index < len(lines) {
        command := strings.Split(lines[index], " ")
        if command[0] == "$" {
            switch command[1] {
                case "cd":
                    curr = curr.getDirectory(command[2])
                case "ls":
                    curr, index = curr.fillDirectory(lines, index + 1)
                default:
                    panic("We shouldn't be seeing this :)")
            }
        }
        index++
    }

    //2. Process directories and calculate size of all directories
    populateSize(curr)
    //3. Return sum of directories with size 100000 or greater
    sum := Sum {
        val: 0,
    }
    sumSizes(curr, &sum)
    return sum.val
}

type Sum struct {
    val int
}

func populateSize(dir Directory) int {
    sum := 0
    for _,sub := range dir.subdirectories {
        sum += populateSize(sub)
    }
    dir.size += sum
    return dir.size
}

func sumSizes(dir Directory, sum *Sum) {
    if dir.size <= 100000 {
        sum.val += dir.size
    }
    for _,sub := range dir.subdirectories {
        sumSizes(sub, sum)
    }
}

func (dir *Directory) fillDirectory(lines []string, index int) (Directory, int) {
    if dir.subdirectories == nil {
        dir.subdirectories = make(map[string]Directory)
    }
    for index < len(lines) {
        entry := strings.Split(lines[index], " ")
        // Next line is a new command
        if entry[0] == "$" {
            return *dir,index
        // Next line is a new directory
        } else if entry[0] == "dir" {
            dir.subdirectories[entry[1]] = Directory {
                subdirectories: make(map[string]Directory),
                size: 0,
                parent: []*Directory{dir},
            }
        // Next line is a file with a size
        } else {
            size,err := strconv.Atoi(entry[0])
            if err != nil {
                fmt.Printf("Failed to parse int from command %v", entry)
                panic("Error parsing!")
            }
            dir.subdirectories[entry[1]] = Directory {
                subdirectories: make(map[string]Directory),
                size: size,
                parent: []*Directory{dir},
            }
        }
        index++
    }

    return *dir,index
}

func (dir *Directory) getDirectory(name string) Directory {
    return dir.subdirectories[name]
}

