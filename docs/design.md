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
Wow, its been a whole month. Incredible. Anyways, I've been thinking about how to store my notes and I've decided to create my own format based on markdown (`.md`) called jotdown (`.jd`). It will describe titles, headings, bold text and other variations in the same way, but reference other files differently.

Parity with md:
- Titles/headings: `# Title` -> `# Very small heading`
- Bold/Italics/Spoilers: `*italics*, **bold**, etc`

Differences:
No html support
Text retains tabs and newlines

### 2023-1-16
Just finished (everything for now at least) the jotdown parser, and it worked out pretty well. My favorite part is that it has NO INVALID STATE! Very cool. Anyways, now that I'm done working that out I'm going to move on to making an editor for it. Before I do that thought, I'm going to make a reverse parser that deserialized jotdown back into text.

### 2023-2-29
I took a little break from this to work on some stuff for Wayland, but I'm now working on the editor. I've run into a bit of an issue with `iced`, which is that the `text_editor` widget always wants to extend as big as it can. I want it to hug its contents.

### 2023-2-31
I DID IT. https://github.com/iced-rs/iced/pull/2221 it was pretty simple in retrospect. I'm now going to get back to my app now :D


