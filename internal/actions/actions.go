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

func DisplayDays(filePath string, daysBack int) {
	chData := loadFile(filePath)
	chrono.ReadDays(chData, daysBack)
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

func AddEntry(filePath string, userText string) {
	//There may be a bug here after the refactor because of os.Exit
	chData := loadFile(filePath)
	if chData == nil {
		chData = chrono.ChronoData{}
	}

	chrono.AppendLogEntry(chData, userText)
	chData.Save(filePath)
}

func AddEntryWithTime(filePath string, userText string, timeText string) {
	//There may be a bug here after the refactor because of os.Exit
	chData := loadFile(filePath)

	_, formatError := chrono.NormalizeTime(timeText)
	if formatError != nil {
		fmt.Println(formatError)
	} else {
		if chData == nil {
			chData = chrono.ChronoData{}
		}

		chrono.AppendLogEntryWithTime(chData, userText, timeText)
		chData.Save(filePath)
	}
}
