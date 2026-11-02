# Chronicler

A command-line journal for recording timestamped notes.

Entries are stored in `chrono.json` under your user config directory (for example, `~/.config/chrono/chrono.json` on Linux).

## Requirements

- Go 1.26.6 or later

## Run

```sh
go run . "wrote project notes"
```

Run without arguments to display today's entries:

```sh
go run .
```

Build an executable with:

```sh
go build .
```

Install it with:

```sh
go install .
```

## Usage

```sh
chronicler "entry text"             # add an entry with the current time
chronicler "entry text" 09:30       # add an entry at a specific time
chronicler "entry text" 09:00 17:00 # add start and end entries
chronicler --today                   # show today's entries
chronicler --month                   # show entries from the current month
chronicler --days 7                  # show the previous 7 days and today
chronicler --edit                    # edit today's entry text
chronicler --editTime                # edit today's entry time
chronicler --delete                  # delete a today's entry
chronicler --version                 # print the version
```

Manual times use 24-hour `HH:MM` format.
