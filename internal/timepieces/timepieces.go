package timepieces

import (
	"fmt"
	"time"
)

type Current struct {
	Year       int
	Month      int
	Day        int
	DateString string
	TimeString string
}

func GetCurrent() Current {
	now := time.Now()
	dateString := now.Format("2006-01-02")
	hour, min, _ := time.Now().Local().Clock()
	timeString := fmt.Sprintf("%d:%02d", hour, min)
	year, month, day := now.Date()

	return Current{
		Year:       year,
		Month:      int(month),
		Day:        day,
		DateString: dateString,
		TimeString: timeString,
	}
}
