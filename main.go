package main

import (
	"chronicler/internal/actions"
	"flag"
	"fmt"
	"os"
	"path/filepath"
)

func hasArg() bool {
	return len(os.Args) > 1
}

const ChronoFileName = "chrono.json"

var ChronoDir string
var ChronoFilePath string

func ensureChronoDir() {
	userDir, _ := os.UserConfigDir()
	ChronoDir = filepath.Join(userDir, "chrono")

	_, dirErr := os.Stat(ChronoDir)

	if dirErr != nil {
		os.MkdirAll(ChronoDir, 0755)
	}
}

func main() {
	ensureChronoDir()
	ChronoFilePath = filepath.Join(ChronoDir, ChronoFileName)

	flag.Usage = func() {
		fmt.Fprintf(os.Stderr, "Usage: chronicler [options] [text]\n\n")
		fmt.Fprintf(os.Stderr, "Examples:\n")
		fmt.Fprintf(os.Stderr, "  chronicler \"wrote project notes\"\n")
		fmt.Fprintf(os.Stderr, "  chronicler --today\n")
		fmt.Fprintf(os.Stderr, "  chronicler --month\n\n")
		fmt.Fprintf(os.Stderr, "Options:\n")
		flag.PrintDefaults()
		info := "\nInfo:\n  Chronicler journal file stored at " + ChronoFilePath + "\n"
		fmt.Fprintf(os.Stderr, info)
	}

	versionFlag := flag.Bool("version", false, "prints version")
	todayFlag := flag.Bool("today", false, "query today's entries")
	monthFlag := flag.Bool("month", false, "query entries for the current month")
	daysFlag := flag.Int("days", 0, "query n days back")
	editFlag := flag.Bool("edit", false, "edit journal")
	deleteFlag := flag.Bool("delete", false, "delete journal entry")
	editTimeFlag := flag.Bool("editTime", false, "edit journal time")

	flag.Parse()

	if hasArg() {
		userText := os.Args[1]

		if *todayFlag {
			actions.DisplayToday(ChronoFilePath)
		} else if *monthFlag {
			actions.DisplayMonth(ChronoFilePath)
		} else if *versionFlag {
			actions.DisplayVersion()
		} else if *editFlag {
			actions.EditEntryText(ChronoFilePath)
		} else if *deleteFlag {
			actions.DeleteEntry(ChronoFilePath)
		} else if *editTimeFlag {
			actions.EditEntryTime(ChronoFilePath)
		} else if *daysFlag > 0 {
			actions.DisplayDays(ChronoFilePath, *daysFlag)
		} else {
			if len(os.Args) > 2 {
				timeArg := os.Args[2]
				actions.AddEntryWithTime(ChronoFilePath, userText, timeArg)
			} else {
				actions.AddEntry(ChronoFilePath, userText)
			}
		}
	} else {
		actions.DisplayToday(ChronoFilePath)
	}
}
