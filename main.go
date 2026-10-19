package main

import (
	"bufio"
	"chronicler/internal/actions"
	"chronicler/internal/chrono"
	"flag"
	"fmt"
	"os"
	"path/filepath"
)

var version string

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

func hasArg() bool {
	return len(os.Args) > 1
}

const ChronoFileName = "chrono.json"

var ChronoDir string
var ChronoFilePath string

func writeLogWithTime(chData chrono.ChronoData, userText string, time string) {
	if chData == nil {
		chData = chrono.ChronoData{}
	}

	chrono.AppendLogEntryWithTime(chData, userText, time)
	chData.Save(ChronoFilePath)
}

func ensureChronoDir() {
	userDir, _ := os.UserConfigDir()
	ChronoDir = filepath.Join(userDir, "chrono")

	_, dirErr := os.Stat(ChronoDir)

	if dirErr != nil {
		os.MkdirAll(ChronoDir, 0755)
	}
}

// Consider testing the cli interface and throw errors for the expected bad flags and such.
// To test, likely need to either override os.Args params, or just pass args to a cli wrapper that knows nothing about args
func main() {
	version = "0.4.0"
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

		chData, err := chrono.Load(ChronoFilePath)
		if err != nil && err != chrono.ErrChronoFileNotFound {
			fmt.Fprintf(os.Stderr, "error: %v\n", err)
			os.Exit(1)
		}

		if *todayFlag {
			actions.DisplayToday(ChronoFilePath)
		} else if *monthFlag {
			actions.DisplayMonth(ChronoFilePath)
		} else if *versionFlag {
			//DisplayVersion
			fmt.Println(version)
		} else if *editFlag {
			//EditEntryText
			chrono.ReadTodayEditMode(chData)
			var index int
			fmt.Scanln(&index)

			fmt.Printf("Editing: ")
			chrono.PrintTodayEditPick(chData, index)

			var saveStr string

			scanner := bufio.NewScanner(os.Stdin)

			if scanner.Scan() {
				saveStr = scanner.Text()
			}

			chrono.EditTodayEntryText(chData, index, saveStr)
			chData.Save(ChronoFilePath)
			chrono.ReadToday(chData)
		} else if *deleteFlag {
			//DeleteEntry
			chrono.ReadTodayEditMode(chData)
			var index int
			fmt.Scanln(&index)

			fmt.Printf("Deleting: ")
			chrono.PrintTodayEditPick(chData, index)

			chrono.DropTodayEntry(chData, index)
			chData.Save(ChronoFilePath)
			chrono.ReadToday(chData)
		} else if *editTimeFlag {
			//EditEntryTime
			chrono.ReadTodayEditMode(chData)
			var index int
			fmt.Scanln(&index)

			fmt.Printf("Editing: ")
			chrono.PrintTodayEditPick(chData, index)

			var saveStr string

			scanner := bufio.NewScanner(os.Stdin)

			if scanner.Scan() {
				saveStr = scanner.Text()
			}

			chrono.EditTodayEntryTime(chData, index, saveStr)
			chData.Save(ChronoFilePath)
			chrono.ReadToday(chData)
		} else if *daysFlag > 0 {
			actions.DisplayDays(ChronoFilePath, *daysFlag)
		} else {
			if len(os.Args) > 2 {
				//AddEntryWithTime
				timeArg := os.Args[2]

				_, formatError := chrono.NormalizeTime(timeArg)
				if formatError != nil {
					fmt.Println(formatError)
				} else {
					writeLogWithTime(chData, userText, timeArg)
				}
			} else {
				actions.AddEntry(ChronoFilePath, userText)
			}
		}

	} else {
		actions.DisplayToday(ChronoFilePath)
	}
}
