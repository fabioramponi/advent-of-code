package main

import (
	"advent-of-code/2015/utils"
	"fmt"
	"regexp"
	"strconv"
)

type instruction struct {
	rhs string
	lhs string
	op  string
	res string
}

var reg map[string]instruction
var strToOp map[string]func(...int) int
var values map[string]int

func instructionsFromString(s string) instruction {
	r := regexp.MustCompile(`(?P<lhs>^\d+|[a-z]{1,2})?\s?((?P<op>AND|LSHIFT|OR|NOT|RSHIFT)?\s(?P<rhs>\d+|[a-z]{1,2}))? -> (?P<r>[a-z]{1,2})`)
	names := r.SubexpNames()
	result := r.FindAllStringSubmatch(s, -1)
	m := map[string]string{}
	for i, n := range result[0] {
		m[names[i]] = n
	}

	res := instruction{}
	if m["op"] != "" {
		res.op = m["op"]
	}
	if m["rhs"] != "" {
		res.rhs = m["rhs"]
	}
	if m["lhs"] != "" {
		res.lhs = m["lhs"]
	}
	res.res = m["r"]

	return res
}

func deref(v string) int {
	val, ok := values[v]
	if ok {
		return val
	}

	inst, ok := reg[v]
	if !ok {
		val := utils.IgnoreError(strconv.Atoi(v))
		values[v] = val
	} else {
		if inst.op != "" {
			if inst.lhs != "" {
				values[v] = strToOp[inst.op](deref(inst.lhs), deref(inst.rhs))
			} else {
				values[v] = strToOp[inst.op](deref(inst.rhs))
			}
		} else {
			utils.Assert(inst.lhs != "")
			values[v] = deref(inst.lhs)
		}
	}
	return values[v]
}

func day07(input []string) []string {
	reg = map[string]instruction{}
	values = map[string]int{}
	strToOp = map[string]func(...int) int{
		"AND": func(in ...int) int {
			utils.Assert(len(in) == 2)
			return in[0] & in[1]
		},
		"OR": func(in ...int) int {
			utils.Assert(len(in) == 2)
			return in[0] | in[1]
		},
		"NOT": func(in ...int) int {
			utils.Assert(len(in) == 1)
			return ^in[0]
		},
		"LSHIFT": func(in ...int) int {
			utils.Assert(len(in) == 2)
			return in[0] << in[1]
		},
		"RSHIFT": func(in ...int) int {
			utils.Assert(len(in) == 2)
			return in[0] >> in[1]
		},
	}

	for _, s := range input {
		instr := instructionsFromString(s)
		reg[instr.res] = instr
	}
	res1 := deref("a")
	reg["b"] = instruction{lhs: strconv.Itoa(res1)}
	for k := range values {
		delete(values, k)
	}
	res2 := deref("a")

	return []string{fmt.Sprintf("answer_1: %d", res1), fmt.Sprintf("answer_2: %d", res2)}
}
