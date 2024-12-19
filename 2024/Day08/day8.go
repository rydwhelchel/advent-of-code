package Day08

import (
	"fmt"
	h "github.com/rydwhelchel/advent-of-code/helpers/go"
	im "image"
	"strconv"
)

type Context struct {
	Path string
}

func (ctx *Context) Name() string { return "Day 8" }

func (ctx *Context) Part1() string {
	grid := h.ReadInputAsByteArray(ctx.Path)
	nodes := initNodeMap(grid)

	antinodes := make(map[string]bool)
	for _, list := range nodes {
		for f := 0; f < len(list)-1; f++ {
			p1 := list[f]
			for s := f + 1; s < len(list); s++ {
				p2 := list[s]
				if anti1 := p2.Mul(2).Sub(p1); h.PointInGrid(anti1, grid) {
					antinodes[fmt.Sprintf("%v", anti1)] = true
				}
				if anti2 := p1.Mul(2).Sub(p2); h.PointInGrid(anti2, grid) {
					antinodes[fmt.Sprintf("%v", anti2)] = true
				}
			}
		}
	}

	return strconv.Itoa(len(antinodes))
}

func initNodeMap(grid [][]byte) map[byte][]im.Point {
	nodes := make(map[byte][]im.Point)
	for y, line := range grid {
		for x, val := range line {
			if val == '.' {
				continue
			}
			if v, ok := nodes[val]; ok {
				nodes[val] = append(v, im.Point{X: x, Y: y})
			} else {
				nodes[val] = []im.Point{{X: x, Y: y}}
			}
		}
	}
	return nodes
}

func (ctx *Context) Part2() string {
	grid := h.ReadInputAsByteArray(ctx.Path)
	nodes := initNodeMap(grid)

	antinodes := make(map[string]bool)
	for _, list := range nodes {
		for f := 0; f < len(list)-1; f++ {
			p1 := list[f]
			for s := f + 1; s < len(list); s++ {
				p2 := list[s]
				cmp1 := p1.Sub(p2)
				cmp2 := p2.Sub(p1)
				// iterate through all points along the slope of p1 - p2
				for i := 0; h.PointInGrid(p1.Sub(cmp2.Mul(i)), grid); i++ {
					poi := p1.Sub(cmp2.Mul(i))
					antinodes[fmt.Sprintf("%v", poi)] = true
				}

				// iterate through all points along the slope of p2 - p1
				for i := 0; h.PointInGrid(p2.Sub(cmp1.Mul(i)), grid); i++ {
					poi := p2.Sub(cmp1.Mul(i))
					antinodes[fmt.Sprintf("%v", poi)] = true
				}
			}
		}
	}

	return strconv.Itoa(len(antinodes))
}
