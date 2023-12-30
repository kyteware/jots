# Jotdown (.jd) format

Jotdown is a docuent format similar to markdown, give or take some features

## Spec

Blocks:
- Paragraphs: `\n\n` seperated single line text blocks. 
  - No hard wrapping. 
  - If single `\n` encountered, seperate into two paragraphs (not recommened but could happen from bad user)
  - Shrink more than two newlines as to not include them in the next paragraph/block.
  - Only paragraphs support text modifiers
  - Ignore starting whitespace (spaces, tabs)
- Titles/Headings: `#` - `######` Single line, doesn't support modifiers
(Add more later)

Text modifiers:
(Add italics, bold etc later)
