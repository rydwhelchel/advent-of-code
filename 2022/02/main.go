package main

import (
    "bufio"
    "log"
    "os"
    "strings"
)

func handleErr(err error) {
    error := log.New(os.Stderr, "ERROR: ", log.LstdFlags)
    error.Printf("%v\n", err)
}

func main() {
    file, err := os.Open("input")
    if err != nil {
        handleErr(err)
        return
    }

    sc := bufio.NewScanner(file)
    games := []Game{}
    for sc.Scan() {
        text := sc.Text()
        raw := strings.Split(text, " ")
        newgame := Game{raw[0], raw[1]}
        games = append(games, newgame)
    }

    part1 := part1(games)
    log.Printf("Part1: %v\n", part1)
    part2 := part2(games)
    log.Printf("Part2: %v\n", part2)
}

type Game struct {
    opp, me string
}

func part1(games []Game) int {
    totalScore := 0
    for _, game := range games {
        totalScore += calculateRound(game)
    }

    return totalScore
}

func part2(games []Game) int {
    totalScore := 0
    for _, game := range games {
        totalScore += throwRound(game)
    }
    return totalScore
}

func throwRound(game Game) int {
    score := 0

    switch game.me {
    case "X":
        score += 0
    case "Y":
        score += 3
    case "Z":
        score += 6
    default:
        println("Heckin errored")
        return 0
}

    switch game.opp {
    case "A":
        switch game.me {
        case "X":
            score += 3
        case "Y":
            score += 1
        case "Z":
            score += 2
        default:
            println("Heckin errored")
            return 0
    }

    case "B":
        switch game.me {
        case "X":
            score += 1
        case "Y":
            score += 2
        case "Z":
            score += 3
        default:
            println("Heckin errored")
            return 0
    }

    case "C":
        switch game.me {
        case "X":
            score += 2
        case "Y":
            score += 3
        case "Z":
            score += 1
        default:
            println("Heckin errored")
            return 0
    }
}
    return score
}

func calculateRound(game Game) int {
    score := 0
    switch game.me {
    case "X":
        score += 1
    case "Y":
        score += 2
    case "Z":
        score += 3
    default:
        println("Heckin errored")
        return 0
}

    switch game.opp {
    case "A":
        switch game.me {
        case "X":
            score += 3
        case "Y":
            score += 6
        case "Z":
            score += 0
        default:
            println("Heckin errored")
            return 0
    }

    case "B":
        switch game.me {
        case "X":
            score += 0
        case "Y":
            score += 3
        case "Z":
            score += 6
        default:
            println("Heckin errored")
            return 0
    }

    case "C":
        switch game.me {
        case "X":
            score += 6
        case "Y":
            score += 0
        case "Z":
            score += 3
        default:
            println("Heckin errored")
            return 0
    }

    default:
        println("Heckin errored")
        return 0
}

    return score
}
