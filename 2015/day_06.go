package main

import (
	"advent-of-code/2015/utils"
	"fmt"
	"regexp"
	"strconv"
)

type operationType string

type squareCoord struct {
	UpLeft    utils.Coord2D
	DownRight utils.Coord2D
}

const (
	turnOn  operationType = "turn on"
	turnOff               = "turn off"
	toggle                = "toggle"
)

type operation struct {
	opType operationType
	square squareCoord
}

func operationFromString(s string) operation {
	r := regexp.MustCompile(`(?P<op>turn on|turn off|toggle) (?P<x1>\d{1,3}),(?P<y1>\d{1,3}) through (?P<x2>\d{1,3}),(?P<y2>\d{1,3})`)
	names := r.SubexpNames()
	result := r.FindAllStringSubmatch(s, -1)
	m := map[string]string{}
	for i, n := range result[0] {
		m[names[i]] = n
	}
	return operation{operationType(m["op"]), squareCoord{utils.Coord2D{X: utils.IgnoreError(strconv.Atoi(m["x1"])), Y: utils.IgnoreError(strconv.Atoi(m["y1"]))}, utils.Coord2D{X: utils.IgnoreError(strconv.Atoi(m["x2"])), Y: utils.IgnoreError(strconv.Atoi(m["y2"]))}}}
}

func parseInput(input *[]string) []operation {
	res := make([]operation, len(*input))
	for i, s := range *input {
		op := operationFromString(s)
		res[i] = op
	}
	return res
}

func day06(input []string) []string {

	operations := parseInput(&input)

	grid1 := make([][]bool, 1000)
	for i := range grid1 {
		grid1[i] = make([]bool, 1000)
	}

	for i, x := range grid1 {
		for j := range x {
			grid1[i][j] = false
		}
	}

	grid2 := make([][]int, 1000)
	for i := range grid2 {
		grid2[i] = make([]int, 1000)
	}

	for _, op := range operations {
		for x := op.square.UpLeft.X; x <= op.square.DownRight.X; x++ {
			for y := op.square.UpLeft.Y; y <= op.square.DownRight.Y; y++ {
				switch op.opType {
				case turnOn:
					grid1[x][y] = true
					grid2[x][y] += 1
				case turnOff:
					grid1[x][y] = false
					if grid2[x][y] > 0 {
						grid2[x][y] -= 1
					}
				case toggle:
					grid1[x][y] = !grid1[x][y]
					grid2[x][y] += 2
				}
			}
		}
	}

	count1 := 0
	count2 := 0
	for _, row := range grid1 {
		count1 += utils.CountIf(row, func(v bool) bool { return v == true })

	}

	for _, row := range grid2 {
		count2 += utils.Sum(row)
	}

	return []string{fmt.Sprintf("answer_1: %d", count1), fmt.Sprintf("answer_2: %d", count2)}
}
