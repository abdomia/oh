// Package reader: read file from disk or from url and download it.
package reader

import (
	"encoding/csv"
	"log"
	"os"
)

func ReadFromDisk(fileName string) [][]string {
	file, err := os.Open(fileName)
	if err != nil {
		log.Fatal(err)
	}

	reader := csv.NewReader(file)
	all, err := reader.ReadAll()
	if err != nil {
		log.Fatal(err)
	}
	return all
}

