
# Description

mdbks is a note-taking utility for curating a book reading list, recording when books are read, and recording thoughts about the books. The frontmatter format should be Obsidian-compatible and based off of my existing book notes template. This is a personal tool catering only to my needs. Intended to replace Obsidian Bases and Books plugin with something smaller.

# Usage

## Create a book note

`mdbks new <query| --isbn isbn>`

1. search Open Library API
2. display top results, prompt for selection
3. if no results -> interactive prompt for required fields
4. generate markdown note with frontmatter
5. open in helix

## update a note's status (to ensure consistent formatting)

`mdbks finish <note title> [--status read|not_finished] [--date YYYY-MM-DD]`

- tab autocomplete note name from books directory
- set status=read (default) or to supplied status
- update date_finished
- prompt to record thoughts
  - if yes -> open note in helix at thoughts heading in insert mode (possible?)

## list books based on status

`mdkbs list [--status STATUS] [--sort FIELD] [--desc]`

- does what it says on the tin

# Implementation Details

- tab completion for update (title) and list (status and fields)
- automatic wiki links for authors and optionally genres
- need to read line number that occurs after Thoughts heading for opening with `hx +LINE file.md`

# Open Questions

- support author/genre search or leave the advanced navigation to pkm
- how to handle mutiple authors
- how to record multiple read events
