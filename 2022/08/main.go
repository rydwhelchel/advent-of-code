package main

import "fmt"

type Super struct {
    sub map[string]Super
    parent []Super
}

func main() {
    t := Super {
        sub: make(map[string]Super),
        parent: []Super{},
    }

    entry := []string{"0"}

    t.addTo(entry)
    fmt.Printf("T: %v", t)
}

func (t *Super) addTo(entry []string) {
    t.sub[entry[0]] = Super {
        sub: make(map[string]Super),
        parent: []Super{*t},
    }
}
