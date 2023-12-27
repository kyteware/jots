# Design Process

## Brainstorming

#### 2023-11-26
Problem Statement: Having all my productive tools in one place is hard
- Its inconvenient to link thoughts together
- I often can't access my notes on all my devices
- I don't really want to share my personal thoughts with Google

The goal of Jots is to help people get everything they possibly can out of their most import tool: their brain.

Jots should support the following features:

- [ ] Notes
  - [ ] Text
  - [ ] Images
  - [ ] Media
  - [ ] User Organization
  - [ ] Sketches
- [ ] Journal
- [ ] Calendar
  - [ ] Sync with other calendars
- [ ] Self-hosting
- [ ] Server hosting

## Ideating

#### 2023-11-26
Here is my initial design: (images/sketch.jpg)[sketch]. Some would call it an artistic masterpiece, but really, its just a first look.

#### 2023-12-26
Wow, its been a whole month. Incredible. Anyways, I've been thinking about how to store my notes and I've decided to create my own format based on markdown (`.md`) called jotdown (`.jd`). It will describe titles, headers, bold text and other variations in the same way, but reference other files differently.

Parity with md:
- Titles/headers: `# Title` -> `# Very small header`
- Bold/Italics/Spoilers: `*italics*, **bold**, etc`

Differences:
No html support
Text retains tabs and newlines


