package utils

import (
	"io/ioutil"
	"strings"
)

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

func MinOf(vars ...int) int {
	min := vars[0]

	for _, i := range vars {
		if min > i {
			min = i
		}
	}

	return min
}
