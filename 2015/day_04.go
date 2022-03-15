package main

import (
	"crypto/md5"
	"fmt"
	"io"
	"strconv"
	"strings"
)

func day04(input []string) []string {
	return []string{day04_impl(input, 1), day04_impl(input, 2)}
}

func day04_impl(input []string, part int) string {
	h := md5.New()
	num := 0
	to_match := strings.Repeat("0", 4+part)
	for ; ; num += 1 {
		h.Reset()
		io.WriteString(h, input[0])
		io.WriteString(h, strconv.Itoa(num))
		res := fmt.Sprintf("%x", h.Sum(nil))
		if res[0:4+part] == to_match {
			break
		}
	}
	return fmt.Sprintf("answer_%d: %d", part, num)
}
