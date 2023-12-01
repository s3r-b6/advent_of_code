package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func main() {
	fileReader, err := os.Open("./input.txt")
	check(err)

	fileScanner := bufio.NewScanner(fileReader)
	fileScanner.Split(bufio.ScanLines)

	var curr, max1, max2, max3 uint64 = 0, 0, 0, 0
	for fileScanner.Scan() {
		if fileScanner.Text() == "" {
			if curr > max1 {
				max3 = max2
				max2 = max1
				max1 = curr
			} else if curr > max2 {
				max3 = max2
				max2 = curr
			} else if curr > max3 {
				max3 = curr
			}
			curr = 0
		} else {
			x, err := strconv.ParseUint(fileScanner.Text(), 0, 64)
			check(err)
			curr += x
		}
	}

	fmt.Printf("Max is: %d\n", max1)
	fmt.Printf("Max3 are: %d %d %d\n", max1, max2, max3)
	fmt.Printf("Max3 sum is: %d\n", max1+max2+max3)

	check(fileReader.Close())
}
