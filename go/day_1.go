package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type elf struct {
	calories int
}

func main() {
	path := "./day_1_input.txt"

	f, err := os.Open(path)

	if err != nil {
		fmt.Println("Cannot open input file")
		return
	}

	var elves []elf
	elves = append(elves, elf{calories: 0})
	index := 0

	scanner := bufio.NewScanner(f)

	for scanner.Scan() {
		line := scanner.Text()

		line = strings.TrimSpace(line)

		if line == "" {
			elves = append(elves, elf{calories: 0})
			index += 1
		} else {
			calories, err := strconv.Atoi(line)
			if err != nil {
				fmt.Println("Could not format calorie number")
				return
			}

			elves[index].calories += calories
		}
	}

	maxCalories := 0

	for elfIndex := range elves {
		if elves[elfIndex].calories > maxCalories {
			maxCalories = elves[elfIndex].calories
		}
	}

	fmt.Println(maxCalories)

}
