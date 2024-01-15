use super::*;

#[test]
fn paragraphs_2newlines_end() {
    let input = "This is a paragraph.\n\nThis is another paragraph.\n\nAnother paragraph!\n\n";
    let expected = vec![
        JdElement::Paragraph(vec![JdTextMod::Normal("This is a paragraph.")]),
        JdElement::Paragraph(vec![JdTextMod::Normal("This is another paragraph.")]),
        JdElement::Paragraph(vec![JdTextMod::Normal("Another paragraph!")]),
    ];
    let (_, output) = parse_jotdown(input).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn paragraphs_noend() {
    let input = "This is a paragraph.\n\nThis is another paragraph.\n";
    let expected = vec![
        JdElement::Paragraph(vec![JdTextMod::Normal("This is a paragraph.")]),
        JdElement::Paragraph(vec![JdTextMod::Normal("This is another paragraph.")]),
    ];
    let (_, output) = parse_jotdown(input).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn paragraph_unterminated() {
    let input = "This is a paragraph.\n\nThis is another paragraph.";
    let expected = vec![
        JdElement::Paragraph(vec![JdTextMod::Normal("This is a paragraph.")]),
        JdElement::Paragraph(vec![JdTextMod::Normal("This is another paragraph.")]),
    ];
    let (_, output) = parse_jotdown(input).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn paragraph_after_newlines() {
    let input = "\n\nThis is a paragraph.\n\nThis is another paragraph.";
    let expected = vec![
        JdElement::Paragraph(vec![JdTextMod::Normal("This is a paragraph.")]),
        JdElement::Paragraph(vec![JdTextMod::Normal("This is another paragraph.")]),
    ];
    let (_, output) = parse_jotdown(input).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn header() {
    let input = "## This is a header\nThis is a paragraph.";
    let expected = vec![
        JdElement::TitleOrHeading((vec![JdTextMod::Normal("This is a header")], 2)),
        JdElement::Paragraph(vec![JdTextMod::Normal("This is a paragraph.")]),
    ];
    let (_, output) = parse_jotdown(input).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn titles() {
    let input = "# This is a title\n\n\nThis is a paragraph.\n\n# This is another title\n";
    let expected = vec![
        JdElement::TitleOrHeading((vec![JdTextMod::Normal("This is a title")], 1)),
        JdElement::Paragraph(vec![JdTextMod::Normal("This is a paragraph.")]),
        JdElement::TitleOrHeading((vec![JdTextMod::Normal("This is another title")], 1)),
    ];
    let (_, output) = parse_jotdown(input).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn paragraph_startswith_hashtag() {
    let input = "#This is not a header";
    let expected = vec![JdElement::Paragraph(vec![JdTextMod::Normal("#This is not a header")])];
    let (_, output) = parse_jotdown(input).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn unordered_list() {
    let input = "- This is a list item\n- This is another list item\n";
    let expected = vec![
        JdElement::UnorderedList(vec![vec![JdTextMod::Normal("This is a list item")], vec![JdTextMod::Normal("This is another list item")]]),
    ];
    let (_, output) = parse_jotdown(input).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn paragraph_and_unordered_list() {
    let input = "This is a paragraph.\n\n- This is a list item\n- This is another list item\n";
    let expected = vec![
        JdElement::Paragraph(vec![JdTextMod::Normal("This is a paragraph.")]),
        JdElement::UnorderedList(vec![vec![JdTextMod::Normal("This is a list item")], vec![JdTextMod::Normal("This is another list item")]]),
    ];
    let (_, output) = parse_jotdown(input).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn unordered_list_and_paragraph() {
    let input = "- This is a list item\n- This is another list item\nThis is a paragraph.";
    let expected = vec![
        JdElement::UnorderedList(vec![vec![JdTextMod::Normal("This is a list item")], vec![JdTextMod::Normal("This is another list item")]]),
        JdElement::Paragraph(vec![JdTextMod::Normal("This is a paragraph.")]),
    ];
    let (_, output) = parse_jotdown(input).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn paragraph_startswith_dash() {
    let input = "-This is not a list item";
    let expected = vec![JdElement::Paragraph(vec![JdTextMod::Normal("-This is not a list item")])];
    let (_, output) = parse_jotdown(input).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn bullet_in_header() {
    let input = "# This is a header with a bullet - in it\nThis is a paragraph.";
    let expected = vec![
        JdElement::TitleOrHeading((vec![JdTextMod::Normal("This is a header with a bullet - in it")], 1)),
        JdElement::Paragraph(vec![JdTextMod::Normal("This is a paragraph.")]),
    ];
    let (_, output) = parse_jotdown(input).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn unordered_list_far() {
    let input = "This is a paragraph.\n\n- This is a list item\n- This is another list item\n\nThis is another paragraph.\n\n";
    let expected = vec![
        JdElement::Paragraph(vec![JdTextMod::Normal("This is a paragraph.")]),
        JdElement::UnorderedList(vec![vec![JdTextMod::Normal("This is a list item")], vec![JdTextMod::Normal("This is another list item")]]),
        JdElement::Paragraph(vec![JdTextMod::Normal("This is another paragraph.")]),
    ];
    let (_, output) = parse_jotdown(input).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn ordered_list() {
    let input = "1. This is a list item\n2. This is another list item\n";
    let expected = vec![
        JdElement::OrderedList(vec![vec![JdTextMod::Normal("This is a list item")], vec![JdTextMod::Normal("This is another list item")]]),
    ];
    let (_, output) = parse_jotdown(input).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn two_ordered_lists() {
    let input = "1. This is a list item\n2. This is another list item\n1. This is a list item\n2. This is another list item\n";
    let expected = vec![
        JdElement::OrderedList(vec![vec![JdTextMod::Normal("This is a list item")], vec![JdTextMod::Normal("This is another list item")]]),
        JdElement::OrderedList(vec![vec![JdTextMod::Normal("This is a list item")], vec![JdTextMod::Normal("This is another list item")]]),
    ];
    let (_, output) = parse_jotdown(input).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn ordered_list_and_paragraph() {
    let input = "1. This is a list item\n2. This is another list item\n\nThis is a paragraph.\n";
    let expected = vec![
        JdElement::OrderedList(vec![vec![JdTextMod::Normal("This is a list item")], vec![JdTextMod::Normal("This is another list item")]]),
        JdElement::Paragraph(vec![JdTextMod::Normal("This is a paragraph.")]),
    ];
    let (_, output) = parse_jotdown(input).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn ordered_and_unordered_lists() {
    let input = "1. This is a list item\n2. This is another list item\n\n- This is a list item\n- This is another list item\n";
    let expected = vec![
        JdElement::OrderedList(vec![vec![JdTextMod::Normal("This is a list item")], vec![JdTextMod::Normal("This is another list item")]]),
        JdElement::UnorderedList(vec![vec![JdTextMod::Normal("This is a list item")], vec![JdTextMod::Normal("This is another list item")]]),
    ];
    let (_, output) = parse_jotdown(input).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn very_long_gaps() {
    let input = "\n\n\n\n\n# This is a very long gap\n\n\n\n\n\n\n\n\n\nI sure hope it doesn't break my parser\n\n\n\n\n\n";
    let expected = vec![
        JdElement::TitleOrHeading((vec![JdTextMod::Normal("This is a very long gap")], 1)),
        JdElement::Paragraph(vec![JdTextMod::Normal("I sure hope it doesn't break my parser")]),
    ];
    let (_, output) = parse_jotdown(input).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn no_jotdown() {
    let input = "\n \n  \n\n\t\n\t\t\n\t\t\t";
    let expected = vec![];
    let (_, output) = parse_jotdown(input).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn checklist() {
    let input = "- [ ] This is a list item\n- [x] This is another list item\n";
    let expected = vec![
        JdElement::Checklist(vec![
            (vec![JdTextMod::Normal("This is a list item")], false),
            (vec![JdTextMod::Normal("This is another list item")], true),
        ]),
    ];
    let (_, output) = parse_jotdown(input).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn checklist_doesnt_eat_unordered() {
    let input = "- [ ] This is a list item\n- [x] This is another list item\n- This is a list item\n- This is another list item\n";
    let expected = vec![
        JdElement::Checklist(vec![
            (vec![JdTextMod::Normal("This is a list item")], false),
            (vec![JdTextMod::Normal("This is another list item")], true),
        ]),
        JdElement::UnorderedList(vec![vec![JdTextMod::Normal("This is a list item")], vec![JdTextMod::Normal("This is another list item")]]),
    ];
    let (_, output) = parse_jotdown(input).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn unordered_doesnt_eat_checklist() {
    let input = "- This is a list item\n- This is another list item\n- [ ] This is a list item\n- [x] This is another list item\n";
    let expected = vec![
        JdElement::UnorderedList(vec![vec![JdTextMod::Normal("This is a list item")], vec![JdTextMod::Normal("This is another list item")]]),
        JdElement::Checklist(vec![
            (vec![JdTextMod::Normal("This is a list item")], false),
            (vec![JdTextMod::Normal("This is another list item")], true),
        ]),
    ];
    let (_, output) = parse_jotdown(input).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn checklist_and_paragraph() {
    let input = "- [ ] This is a list item\n- [x] This is another list item\n\nThis is a paragraph.\n";
    let expected = vec![
        JdElement::Checklist(vec![
            (vec![JdTextMod::Normal("This is a list item")], false),
            (vec![JdTextMod::Normal("This is another list item")], true),
        ]),
        JdElement::Paragraph(vec![JdTextMod::Normal("This is a paragraph.")]),
    ];
    let (_, output) = parse_jotdown(input).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn checklist_and_ordered_list() {
    let input = "- [ ] This is a list item\n- [x] This is another list item\n\n1. This is a list item\n2. This is another list item\n";
    let expected = vec![
        JdElement::Checklist(vec![
            (vec![JdTextMod::Normal("This is a list item")], false),
            (vec![JdTextMod::Normal("This is another list item")], true),
        ]),
        JdElement::OrderedList(vec![vec![JdTextMod::Normal("This is a list item")], vec![JdTextMod::Normal("This is another list item")]]),
    ];
    let (_, output) = parse_jotdown(input).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn checklist_and_unordered_list() {
    let input = "- [ ] This is a list item\n- [x] This is another list item\n\n- This is a list item\n- This is another list item\n";
    let expected = vec![
        JdElement::Checklist(vec![
            (vec![JdTextMod::Normal("This is a list item")], false),
            (vec![JdTextMod::Normal("This is another list item")], true),
        ]),
        JdElement::UnorderedList(vec![vec![JdTextMod::Normal("This is a list item")], vec![JdTextMod::Normal("This is another list item")]]),
    ];
    let (_, output) = parse_jotdown(input).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn checklist_and_checklist() {
    let input = "- [ ] This is a list item\n- [x] This is another list item\n\n- [ ] This is a list item\n- [x] This is another list item\n";
    let expected = vec![
        JdElement::Checklist(vec![
            (vec![JdTextMod::Normal("This is a list item")], false),
            (vec![JdTextMod::Normal("This is another list item")], true),
        ]),
        JdElement::Checklist(vec![
            (vec![JdTextMod::Normal("This is a list item")], false),
            (vec![JdTextMod::Normal("This is another list item")], true),
        ]),
    ];
    let (_, output) = parse_jotdown(input).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn embed() {
    let input = "[https://www.youtube.com/watch?v=dQw4w9WgXcQ]";
    let expected = vec![JdElement::Embed("https://www.youtube.com/watch?v=dQw4w9WgXcQ")];
    let (_, output) = parse_jotdown(input).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn paragraph_and_embed() {
    let input = "This is a paragraph.\n\n[https://www.youtube.com/watch?v=dQw4w9WgXcQ]";
    let expected = vec![
        JdElement::Paragraph(vec![JdTextMod::Normal("This is a paragraph.")]),
        JdElement::Embed("https://www.youtube.com/watch?v=dQw4w9WgXcQ"),
    ];
    let (_, output) = parse_jotdown(input).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn list_embed_other_list() {
    let input = "- [ ] This is a list item\n- [x] This is another list item\n[https://www.youtube.com/watch?v=dQw4w9WgXcQ]\n- This is a list item\n- This is another list item\n";
    let expected = vec![
        JdElement::Checklist(vec![
            (vec![JdTextMod::Normal("This is a list item")], false),
            (vec![JdTextMod::Normal("This is another list item")], true),
        ]),
        JdElement::Embed("https://www.youtube.com/watch?v=dQw4w9WgXcQ"),
        JdElement::UnorderedList(vec![vec![JdTextMod::Normal("This is a list item")], vec![JdTextMod::Normal("This is another list item")]]),
    ];
    let (_, output) = parse_jotdown(input).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn several_embeds() {
    let input = "[https://www.youtube.com/watch?v=dQw4w9WgXcQ]\n[https://www.youtube.com/watch?v=dQw4w9WgXcQ]\n[https://www.youtube.com/watch?v=dQw4w9WgXcQ]\n";
    let expected = vec![
        JdElement::Embed("https://www.youtube.com/watch?v=dQw4w9WgXcQ"),
        JdElement::Embed("https://www.youtube.com/watch?v=dQw4w9WgXcQ"),
        JdElement::Embed("https://www.youtube.com/watch?v=dQw4w9WgXcQ"),
    ];
    let (_, output) = parse_jotdown(input).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn title_paragraph_embed() {
    let input = "# This is a title\n\nThis is a paragraph.\n\n[https://www.youtube.com/watch?v=dQw4w9WgXcQ]\n";
    let expected = vec![
        JdElement::TitleOrHeading((vec![JdTextMod::Normal("This is a title")], 1)),
        JdElement::Paragraph(vec![JdTextMod::Normal("This is a paragraph.")]),
        JdElement::Embed("https://www.youtube.com/watch?v=dQw4w9WgXcQ"),
    ];
    let (_, output) = parse_jotdown(input).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn title_formatted() {
    let input = "# This is a **bold**, but *itatic title*\n\n## This header has `raw text` in it";

    let expected = vec![
        JdElement::TitleOrHeading((
            vec![
                JdTextMod::Normal("This is a "),
                JdTextMod::Bold("bold"),
                JdTextMod::Normal(", but "),
                JdTextMod::Italic("itatic title"),
            ],
            1,
        )),
        JdElement::TitleOrHeading((vec![JdTextMod::Normal("This header has "), JdTextMod::Raw("raw text"), JdTextMod::Normal(" in it")], 2)),
    ];
    let (_, output) = parse_jotdown(input).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn checklist_formatted() {
    let input = "- [ ] This is a **bold**, but *itatic checkbox*\n- [x] This marked checkbox has `raw text` in it";

    let expected = vec![JdElement::Checklist(vec![
        (
            vec![
                JdTextMod::Normal("This is a "),
                JdTextMod::Bold("bold"),
                JdTextMod::Normal(", but "),
                JdTextMod::Italic("itatic checkbox"),
            ],
            false,
        ),
        (
            vec![JdTextMod::Normal("This marked checkbox has "), JdTextMod::Raw("raw text"), JdTextMod::Normal(" in it")],
            true,
        ),
    ])];
    let (_, output) = parse_jotdown(input).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn code_block() {
    let input = "```\nThis is a code block\n```\n";
    let expected = vec![JdElement::CodeBlock(("This is a code block\n", None))];
    let (_, output) = parse_jotdown(input).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn code_block_python() {
    let input = "```python\nprint('This is a code block')\n```\n";
    let expected = vec![JdElement::CodeBlock(("print('This is a code block')\n", Some("python")))];
    let (_, output) = parse_jotdown(input).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn multiline_code_block() {
    let input = "```\nThis is a code block\nThis is a code block\n```\n";
    let expected = vec![JdElement::CodeBlock(("This is a code block\nThis is a code block\n", None))];
    let (_, output) = parse_jotdown(input).unwrap();
    assert_eq!(output, expected);

}

#[test]
fn code_block_and_paragraph() {
    let input = "```\nThis is a code block\n```\n\n**This is bold text**\n";
    let expected = vec![
        JdElement::CodeBlock(("This is a code block\n", None)),
        JdElement::Paragraph(vec![JdTextMod::Bold("This is bold text")]),
    ];
    let (_, output) = parse_jotdown(input).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn title_and_code_block() {
    let input = "# This is a title\n\n```\nThis is a code block\n```\n";
    let expected = vec![
        JdElement::TitleOrHeading((vec![JdTextMod::Normal("This is a title")], 1)),
        JdElement::CodeBlock(("This is a code block\n", None)),
    ];
    let (_, output) = parse_jotdown(input).unwrap();
    assert_eq!(output, expected);
}