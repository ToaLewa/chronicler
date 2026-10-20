package actions

import (
	"strings"
	"testing"
)

func TestAddEntryWithTimeValidatesTimeString(t *testing.T) {
	err := AddEntryWithTime("../../test/test_chrono.json", "test", "invalid")

	if !strings.Contains(err.Error(), "does not conform to hh:mm format") {
		t.Errorf("AddEntryWithTime did not check invalid timestamp")
	}
}
