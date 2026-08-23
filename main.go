package main

import (
	"chronicler/internal/chrono"
	"chronicler/internal/timepieces"
	"encoding/json"
	"fmt"
	"os"
	"strconv"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

type ChronoMarkdownFile struct {
	name        string
	frontmatter frontmatter
}

type frontmatter struct {
	prev    string
	journal string
	next    string
}

// type content struct {
// 	header  string
// 	bullets []Entry
// }

func createFirst(fileName string) {
	//Unimplemented
	fmt.Println(fileName)
}

func hasArg() bool {
	return len(os.Args) > 1
}

const ChronoFileName = "chrono.json"

func main() {

	if hasArg() {
		userText := os.Args[1]

		chFile, err := chrono.Load(ChronoFileName)
		if err != nil && err != chrono.ErrChronoFileNotFound {
			fmt.Fprintf(os.Stderr, "error: %v\n", err)
			os.Exit(1)
		}

		if chFile == nil {
			chFile = chrono.ChronoFile{}
		}

		chrono.AppendLogEntry(chFile, userText)

		b, err := json.Marshal(chFile)
		if err != nil {
			fmt.Fprintf(os.Stderr, "error: encode %s: %v\n", ChronoFileName, err)
			os.Exit(1)
		}

		writeErr := os.WriteFile(ChronoFileName, b, 0666)

		if writeErr != nil {
			fmt.Fprintf(os.Stderr, "error: write %s: %v\n", ChronoFileName, err)
			os.Exit(1)
		}

		fmt.Println("Write file")

	} else {
		timePieces := timepieces.GetCurrentTimePieces()
		fmt.Println(timePieces.DateString)

		// Read the current directory (".")
		entries, err := os.ReadDir("/Users/kkulis/Documents/atg/chrono/")
		fileCount := len(entries)
		check(err)

		if fileCount > 0 {
			fmt.Println("Files found")
			fmt.Println(strconv.Itoa(fileCount))

			fileName := "chronicle-" + timePieces.DateString + ".md"
			createFirst(fileName)

		}
		// if err != nil {
		// 	log.Fatal(err)
		// }
		//
		// for _, entry := range entries {
		// 	// Output the name and whether it's a directory
		// 	fmt.Printf("Name: %s | IsDir: %t\n", entry.Name(), entry.IsDir())
		// }

		path := "/Users/kkulis/Documents/atg/chrono/chronicle-2026-08-18.md"
		dat, err := os.ReadFile(path)
		check(err)

		fmt.Println(dat)
		// f, err := os.Open(path)
		// check(err)

		// fmt.Print(string(dat))
		// fmt.Println("Test")
	}
}
