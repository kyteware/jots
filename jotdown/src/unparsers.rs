use crate::{JdText, JdTextMod};

pub fn unparse_title_heading(output: &mut String, text: &JdText, num_hashtags: u8) {
    for _ in 0..num_hashtags {
        output.push('#');
    }
    output.push(' ');
    unparse_text(output, text);
    output.push('\n');
}

pub fn unparse_code_block(output: &mut String, language: Option<&str>, contents: &str) {
    output.push_str("```");
    output.push_str(language.unwrap_or(""));
    output.push_str("\n");
    output.push_str(contents);
    output.push_str("```\n");
}

pub fn unparse_embed(output: &mut String, embed: &str) {
    output.push_str("[");
    output.push_str(embed);
    output.push_str("]\n");
}

pub fn unparse_checklist(output: &mut String, checklist: &[(JdText, bool)]) {
    for (item, checked) in checklist {
        output.push_str("- [");
        if *checked {
            output.push('x');
        } else {
            output.push(' ');
        }
        output.push_str("] ");
        unparse_text(output, &item);
        output.push('\n');
    }
}

pub fn unparse_unordered_list(output: &mut String, list: &[JdText]) {
    for item in list {
        output.push_str("- ");
        unparse_text(output, item);
        output.push('\n');
    }
}

pub fn unparse_ordered_list(output: &mut String, list: &[JdText]) {
    for (i, item) in list.iter().enumerate() {
        output.push_str(&format!("{}. ", i + 1));
        unparse_text(output, item);
        output.push('\n');
    }
}

pub fn unparse_paragraph(output: &mut String, paragraph: &JdText) {
    unparse_text(output, paragraph);
    output.push('\n');
}

fn unparse_text(output: &mut String, text: &JdText) {
    for modded in text {
        match modded {
            JdTextMod::Normal(text) => output.push_str(text),
            JdTextMod::Bold(text) => {
                output.push_str("**");
                output.push_str(text);
                output.push_str("**");
            }
            JdTextMod::Italic(text) => {
                output.push_str("*");
                output.push_str(text);
                output.push_str("*");
            }
            JdTextMod::Raw(text) => {
                output.push_str("`");
                output.push_str(text);
                output.push_str("`");
            }
            JdTextMod::Reference((descriptor, link)) => {
                output.push_str("[");
                output.push_str(descriptor);
                output.push_str("](");
                output.push_str(link);
                output.push_str(")");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::JdTextMod;

    #[test]
    fn up_title_heading() {
        let mut output = String::new();
        super::unparse_title_heading(&mut output, &vec![JdTextMod::Bold("Hello")], 1);
        assert_eq!(output, "# **Hello**\n");
    }

    #[test]
    fn up_code_block() {
        let mut output = String::new();
        super::unparse_code_block(&mut output, Some("rust"), "fn main() {}\n");
        assert_eq!(output, "```rust\nfn main() {}\n```\n");
    }

    #[test]
    fn up_embed() {
        let mut output = String::new();
        super::unparse_embed(&mut output, "https://www.youtube.com/watch?v=dQw4w9WgXcQ");
        assert_eq!(output, "[https://www.youtube.com/watch?v=dQw4w9WgXcQ]\n");
    }

    #[test]
    fn up_checklist() {
        let mut output = String::new();
        super::unparse_checklist(
            &mut output,
            &vec![
                (vec![JdTextMod::Normal("Hello")], true),
                (vec![JdTextMod::Normal("World")], false),
            ],
        );
        assert_eq!(output, "- [x] Hello\n- [ ] World\n");
    }

    #[test]
    fn up_unordered_list() {
        let mut output = String::new();
        super::unparse_unordered_list(
            &mut output,
            &vec![vec![JdTextMod::Raw("Hello")], vec![JdTextMod::Italic("World")]],
        );
        assert_eq!(output, "- `Hello`\n- *World*\n");
    }

    #[test]
    fn up_ordered_list() {
        let mut output = String::new();
        super::unparse_ordered_list(
            &mut output,
            &vec![vec![JdTextMod::Bold("boop "), JdTextMod::Raw("Hello")], vec![JdTextMod::Italic("World")]],
        );
        assert_eq!(output, "1. **boop **`Hello`\n2. *World*\n");
    }

    #[test]
    fn up_paragraph() {
        let mut output = String::new();
        super::unparse_paragraph(
            &mut output,
            &vec![JdTextMod::Bold("boop "), JdTextMod::Raw("Hello"), JdTextMod::Italic("World")],
        );
        assert_eq!(output, "**boop **`Hello`*World*\n");
    }
}