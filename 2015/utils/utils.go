package utils

import (
	"io/ioutil"
	"strings"

	"golang.org/x/exp/constraints"
)

type Number interface {
	constraints.Float | constraints.Integer | constraints.Complex
}

type Coord2D struct {
	X int
	Y int
}

func Check(err error) {
	if err != nil {
		panic(err)
	}
}

func ReadInput(filename string) []string {
	data, err := ioutil.ReadFile(filename)
	Check(err)
	return strings.Split(string(data), "\n")
}

func MinOf[V constraints.Ordered](vars ...V) V {
	min := vars[0]

	for _, i := range vars {
		if min > i {
			min = i
		}
	}

	return min
}

func MaxOf[V constraints.Ordered](vars ...V) V {
	max := vars[0]

	for _, i := range vars {
		if max < i {
			max = i
		}
	}

	return max
}

func Unique[V comparable](vSlice []V) []V {
	keys := make(map[V]bool)
	list := []V{}
	for _, entry := range vSlice {
		if _, value := keys[entry]; !value {
			keys[entry] = true
			list = append(list, entry)
		}
	}
	return list
}

func CountIf[V any](vSlice []V, predicate func(V) bool) int {
	count := 0
	for _, v := range vSlice {
		if predicate(v) {
			count += 1
		}
	}
	return count
}

func Sum[V Number](vSlice []V) V {
	var sum V = 0
	for _, v := range vSlice {
		sum += v

	}
	return sum
}

func IsBetween[V constraints.Ordered](value V, bound1 V, bound2 V) bool {
	return value < MaxOf(bound1, bound2) && value > MinOf(bound1, bound2)
}

func IgnoreError[retType interface{}](ret retType, err error) retType {
	Check(err)
	return ret
}
