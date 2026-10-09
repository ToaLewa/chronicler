package main

import (
	"bufio"
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

func writeLog(chData chrono.ChronoData, userText string) {
	if chData == nil {
		chData = chrono.ChronoData{}
	}

	chrono.AppendLogEntry(chData, userText)
	chData.Save(ChronoFilePath)
}

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

func main() {
	version = "0.2.0"
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
			chrono.ReadToday(chData)
		} else if *monthFlag {
			chrono.ReadMonth(chData)
		} else if *versionFlag {
			fmt.Println(version)
		} else if *editFlag {
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
		} else if *editTimeFlag {
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
			chrono.ReadDays(chData, *daysFlag)
		} else {
			if len(os.Args) > 2 {
				writeLogWithTime(chData, userText, os.Args[2])
			} else {
				writeLog(chData, userText)
			}
		}

	} else {
		chData, err := chrono.Load(ChronoFilePath)
		if err != nil && err != chrono.ErrChronoFileNotFound {
			fmt.Fprintf(os.Stderr, "error: %v\n", err)
			os.Exit(1)
		}

		chrono.ReadToday(chData)
	}
}
