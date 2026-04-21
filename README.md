# Description

mdbks is a note-taking utility for curating a book reading list, recording when books are read, and recording thoughts about the books. The frontmatter format should be Obsidian-compatible and based off of my existing book notes template. This is a personal tool catering only to my needs. Intended to replace Obsidian Bases and Books plugin with something smaller and bespoke.

# Usage

## `mdbks new <query>`

- [x] search Open Library API
- [x] display top results, prompt for selection
- [x] generate markdown note with frontmatter
- [x] author names formatted as `[[wiki links]]`
- [x] set output path via arg

## `mdbks start <path>`

- [x] set status to 'Reading' and set start date
- [x] optional date override

## `mdbks not/finish <path>`

- [x] set status to 'Read' or 'NotFinished' and set finish date if applicable
- [x] optional finished date override
- [x] open note in helix after update
- [x] open note at ## Thoughts section

## `mdbks reread <path>`

- [ ] append new read session
- [ ] no-op if `Status=ToRead`
- [ ] trigger if `mdbks new` called on existing book

## Planned

- [ ] TODO: tests!
- [ ] `[[series]]` linking
- [ ] set up author notes to be created in /Authors in default dir
- [ ] support parsing OpenLibrary subjects into basic `[[genres]]`
- [ ] TUI w/ fuzzy search and configurable navigation keybindings (replace existing dialogs for consistency)
  - [ ] set output path via config file
  - [ ] if no results -> interactive prompt for required fields
  - [ ] editor set via config file
