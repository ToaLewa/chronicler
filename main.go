package main

import (
	"chronicler/internal/chrono"
	"encoding/json"
	"flag"
	"fmt"
	"os"
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

func writeLog(chData chrono.ChronoData, userText string) {
	if chData == nil {
		chData = chrono.ChronoData{}
	}

	chrono.AppendLogEntry(chData, userText)

	b, err := json.Marshal(chData)
	if err != nil {
		fmt.Fprintf(os.Stderr, "error: encode %s: %v\n", ChronoFileName, err)
		os.Exit(1)
	}

	writeErr := os.WriteFile(ChronoFileName, b, 0666)

	if writeErr != nil {
		fmt.Fprintf(os.Stderr, "error: write %s: %v\n", ChronoFileName, err)
		os.Exit(1)
	}
	fmt.Println("Wrote to chronicler file")
}

func main() {
	flag.Usage = func() {
		fmt.Fprintf(os.Stderr, "Usage: chronicler [options] [text]\n\n")
		fmt.Fprintf(os.Stderr, "Examples:\n")
		fmt.Fprintf(os.Stderr, "  chronicler \"wrote project notes\"\n")
		fmt.Fprintf(os.Stderr, "  chronicler --today\n")
		fmt.Fprintf(os.Stderr, "  chronicler --month\n\n")
		fmt.Fprintf(os.Stderr, "Options:\n")
		flag.PrintDefaults()
	}

	todayFlag := flag.Bool("today", false, "query today's entries")
	monthFlag := flag.Bool("month", false, "query entries for the current month")
	daysFlag := flag.Int("days", 1, "query n days back")

	flag.Parse()

	if hasArg() {
		userText := os.Args[1]

		chData, err := chrono.Load(ChronoFileName)
		if err != nil && err != chrono.ErrChronoFileNotFound {
			fmt.Fprintf(os.Stderr, "error: %v\n", err)
			os.Exit(1)
		}

		if *todayFlag {
			chrono.ReadToday(chData)
		} else if *monthFlag {
			chrono.ReadMonth(chData)
		} else if *daysFlag > 0 {
			chrono.ReadDays(chData, *daysFlag)
		} else {
			writeLog(chData, userText)
		}

	}
}
