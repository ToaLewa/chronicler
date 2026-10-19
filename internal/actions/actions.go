package actions

import (
	"chronicler/internal/chrono"
	"fmt"
	"os"
)

func DisplayDays()  {
	
}

func DisplayToday(filePath string) {
		chData, err := chrono.Load(filePath)
		if err != nil && err != chrono.ErrChronoFileNotFound {
			fmt.Fprintf(os.Stderr, "error: %v\n", err)
			os.Exit(1)
		}

		chrono.ReadToday(chData)
}

func DisplayMonth() {

}

func DeleteEntry()  {
	
}

func EditEntryText()  {
	
}

func EditEntryTime()  {
	
}

func AddEntry()  {
	
}

func AddEntryWithTime()  {
	
}
