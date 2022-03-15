package main

import "fmt"

type coord2D struct {
	x int
	y int
}

func day03(input []string) []string {
	return []string{day03_impl(input, 1), day03_impl(input, 2)}
}

func day03_impl(input []string, part int) string {

	visited := make(map[coord2D]int)
	positions := make([]coord2D, part)
	visited[positions[0]] = part

	for i, c := range input[0] {
		pos := &positions[i%part]
		switch c {
		case '<':
			pos.x = pos.x - 1
		case '>':
			pos.x = pos.x + 1
		case 'v':
			pos.y = pos.y - 1
		case '^':
			pos.y = pos.y + 1
		}
		if val, ok := visited[*pos]; ok {
			val += 1
		} else {
			visited[*pos] = 1
		}
	}

	return fmt.Sprintf("answer_%d: %d", part, len(visited))

}
