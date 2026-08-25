package timepieces

import (
	"fmt"
	"time"
)

type TimePiece struct {
	Year       int
	Month      int
	Day        int
	DateString string
	TimeString string
}

func GetCurrent() TimePiece {
	now := time.Now()
	dateString := now.Format("2006-01-02")
	hour, min, _ := time.Now().Local().Clock()
	timeString := fmt.Sprintf("%d:%02d", hour, min)
	year, month, day := now.Date()

	return TimePiece{
		Year:       year,
		Month:      int(month),
		Day:        day,
		DateString: dateString,
		TimeString: timeString,
	}
}

func Get(then time.Time) TimePiece {
	dateString := then.Format("2006-01-02")
	hour, min, _ := time.Now().Local().Clock()
	timeString := fmt.Sprintf("%d:%02d", hour, min)
	year, month, day := then.Date()

	return TimePiece{
		Year:       year,
		Month:      int(month),
		Day:        day,
		DateString: dateString,
		TimeString: timeString,
	}
}
