package Day14

import (
	"bytes"
	"fmt"
	tea "github.com/charmbracelet/bubbletea"
	h "github.com/rydwhelchel/advent-of-code/helpers/go"
	i "image"
	"slices"
	"strconv"
	"strings"
	"sync"
)

type Context struct {
	Path string
}

func (ctx *Context) Name() string { return "Day 14" }

const (
	GRIDMAXX = 101
	GRIDMAXY = 103
)

func (ctx *Context) Part1() string {
	lines := h.ReadInputAsLines(ctx.Path)

	return strconv.Itoa(getPart1(lines))
}

func getPart1(lines []string) int {
	bots := getBots(lines)
	var wg sync.WaitGroup
	// parallelizing this saves about 1ms 🦀
	//actor model time
	for _, bot := range bots {
		wg.Add(1)
		go func() {
			defer wg.Done()
			bot.moveForSeconds(100)
		}()
	}
	wg.Wait()

	q1 := i.Rect(0, 0, GRIDMAXX/2, GRIDMAXY/2)
	q2 := i.Rect(GRIDMAXX/2+1, 0, GRIDMAXX, GRIDMAXY/2)
	q3 := i.Rect(0, GRIDMAXY/2+1, GRIDMAXX/2, GRIDMAXY)
	q4 := i.Rect(GRIDMAXX/2+1, GRIDMAXY/2+1, GRIDMAXX, GRIDMAXY)
	var q1C, q2C, q3C, q4C int
	for _, bot := range bots {
		if bot.Loc.In(q1) {
			q1C++
		} else if bot.Loc.In(q2) {
			q2C++
		} else if bot.Loc.In(q3) {
			q3C++
		} else if bot.Loc.In(q4) {
			q4C++
		}
	}
	return q1C * q2C * q3C * q4C
}

type Bot struct {
	// trying points instead of my coords
	Loc i.Point
	Vel i.Point
}

func getBots(lines []string) []*Bot {
	var bots []*Bot
	for _, line := range lines {
		var b Bot
		parts := strings.Split(line, " ")

		x := strings.Split(parts[0], "=")
		n := strings.Split(x[1], ",")
		nx, _ := strconv.Atoi(n[0])
		ny, _ := strconv.Atoi(n[1])
		b.Loc = i.Point{X: nx, Y: ny}

		y := strings.Split(parts[1], "=")
		m := strings.Split(y[1], ",")
		mx, _ := strconv.Atoi(m[0])
		my, _ := strconv.Atoi(m[1])
		b.Vel = i.Point{X: mx, Y: my}
		bots = append(bots, &b)
	}

	return bots
}

func (b *Bot) move() {
	b.Loc = b.Loc.Add(b.Vel)
	if b.Loc.X >= GRIDMAXX {
		b.Loc.X -= GRIDMAXX
	} else if b.Loc.X < 0 {
		b.Loc.X += GRIDMAXX
	}
	if b.Loc.Y >= GRIDMAXY {
		b.Loc.Y -= GRIDMAXY
	} else if b.Loc.Y < 0 {
		b.Loc.Y += GRIDMAXY
	}
}

func (b *Bot) moveForSeconds(seconds int) {
	for range seconds {
		b.move()
	}
}

func next(bb []*Bot) {
	for _, b := range bb {
		b.move()
	}
}

type State struct {
	Iteration int
	Entries   []entry
	Locations []i.Point
}

func (st State) Init() tea.Cmd {
	return nil
}

func (st State) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.KeyMsg:
		switch msg.String() {
		case "ctrl+c", "q":
			return st, tea.Quit
		case " ":
			st.Iteration++
			st.Locations = st.Entries[st.Iteration].locs
			return st, nil
		}
	}
	return st, nil
}

func (st State) View() string {
	s := fmt.Sprintf("~~~~ Iteration #%v ~~~~\n", st.Entries[st.Iteration].iteration) // Always shows 0 for iteration

	var grid [][]byte
	for range GRIDMAXY {
		var line = bytes.Repeat([]byte{' '}, GRIDMAXX)
		grid = append(grid, line)
	}
	for _, loc := range st.Locations {
		grid[loc.Y][loc.X] = 'O'
	}
	for _, line := range grid {
		s += string(line) + "\n"
	}

	return s
}

type entry struct {
	centralized int
	iteration   int
	locs        []i.Point
}

func getLocs(bb []*Bot) []i.Point {
	var locs []i.Point
	for _, b := range bb {
		locs = append(locs, b.Loc)
	}
	return locs
}

func (ctx *Context) Part2() string {
	lines := h.ReadInputAsLines(ctx.Path)
	bots := getBots(lines)
	// Assuming the picture will be mostly centered
	center := i.Rect(GRIDMAXX/4, GRIDMAXY/4, 3*GRIDMAXX/4, 3*GRIDMAXY/4)

	var entries []entry

	for x := 1; x < 15000; x++ {
		next(bots)
		centralized := 0
		for _, bot := range bots {
			if bot.Loc.In(center) {
				centralized++
			}
		}
		entries = append(entries, entry{centralized, x, getLocs(bots)})
	}
	slices.SortFunc(entries, func(ent1, ent2 entry) int {
		if ent1.centralized > ent2.centralized {
			return -1
		} else if ent1.centralized < ent2.centralized {
			return 1
		}
		return 0
	})

	// Visualize output
	//st := State{
	//	Iteration: 0,
	//	Entries:   entries,
	//	Locations: entries[0].locs,
	//}
	//
	//p := tea.NewProgram(st)
	//if _, err := p.Run(); err != nil {
	//	fmt.Printf("BIG ERROR UH OH %v", err)
	//	os.Exit(1)
	//}

	return strconv.Itoa(entries[0].iteration)
}
