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

- [x] append new read session
- [x] no-op if `Status=ToRead`

## Planned

- [ ] TODO: tests! (mostly to prevent unnoticed regression)
- [ ] TODO: `[[series]]` linking
  - [ ] add `--series` flag to Command::New
  - [ ] Add `series Option<string>` with serde skip if Option None
  - [ ] Format series as `[[series]]` set on frontmatter
  - [ ] Create series note in `./Series/` after book note, swallow `AlreadyExists`
- [ ] TODO: support parsing OpenLibrary subjects into basic `[[genres]]`
- [ ] TODO: an init command to ensure folder structure (author, series, etc)

## Phase 2

- [ ] TUI w/ fuzzy search and configurable navigation keybindings (replace existing dialogs for consistency)
  - [ ] set output path via config file in order to work when invoked from any dir
  - [ ] if no results -> interactive prompt for required fields
  - [ ] editor set via config file
