package actions

import (
	"chronicler/internal/chrono"
	"fmt"
	"os"
)

func loadFile(filePath string) chrono.ChronoData {
	chData, err := chrono.Load(filePath)
	if err != nil && err != chrono.ErrChronoFileNotFound {
		fmt.Fprintf(os.Stderr, "error: %v\n", err)
		os.Exit(1)
	}

	return chData
}

func DisplayDays() {

}

func DisplayToday(filePath string) {
	chData := loadFile(filePath)
	chrono.ReadToday(chData)
}

func DisplayMonth(filePath string) {
	chData := loadFile(filePath)
	chrono.ReadMonth(chData)
}

func DeleteEntry() {

}

func EditEntryText() {

}

func EditEntryTime() {

}

func AddEntry() {

}

func AddEntryWithTime() {

}
