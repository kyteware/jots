mod model;
mod parsers;

pub use model::JdElement;
use nom::{branch::alt, combinator::map, multi::many0, IResult};
use parsers::{parse_paragraph, parse_title_heading, parse_unordered_list};

pub fn parse_jotdown(input: &str) -> IResult<&str, Vec<JdElement>> {
    let (input, output) = many0(alt((
        map(parse_title_heading, JdElement::TitleOrHeading),
        map(parse_unordered_list, JdElement::UnorderedList),
        map(parse_paragraph, JdElement::Paragraph),
    )))(input)?;

    Ok((input, output))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paragraphs_2newlines_end() {
        let input = "This is a paragraph.\n\nThis is another paragraph.\n\nAnother paragraph!\n\n";
        let expected = vec![
            JdElement::Paragraph("This is a paragraph."),
            JdElement::Paragraph("This is another paragraph."),
            JdElement::Paragraph("Another paragraph!"),
        ];
        let (_, output) = parse_jotdown(input).unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn paragraphs_noend() {
        let input = "This is a paragraph.\n\nThis is another paragraph.\n";
        let expected = vec![
            JdElement::Paragraph("This is a paragraph."),
            JdElement::Paragraph("This is another paragraph."),
        ];
        let (_, output) = parse_jotdown(input).unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn header() {
        let input = "## This is a header\nThis is a paragraph.";
        let expected = vec![
            JdElement::TitleOrHeading(("This is a header", 2)),
            JdElement::Paragraph("This is a paragraph."),
        ];
        let (_, output) = parse_jotdown(input).unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn titles() {
        let input = "# This is a title\n\n\nThis is a paragraph.\n\n# This is another title\n";
        let expected = vec![
            JdElement::TitleOrHeading(("This is a title", 1)),
            JdElement::Paragraph("This is a paragraph."),
            JdElement::TitleOrHeading(("This is another title", 1)),
        ];
        let (_, output) = parse_jotdown(input).unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn paragraph_startswith_hashtag() {
        let input = "#This is not a header";
        let expected = vec![JdElement::Paragraph("#This is not a header")];
        let (_, output) = parse_jotdown(input).unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn unordered_list() {
        let input = "- This is a list item\n- This is another list item\n";
        let expected = vec![
            JdElement::UnorderedList(vec!["This is a list item", "This is another list item"]),
        ];
        let (_, output) = parse_jotdown(input).unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn paragraph_and_unordered_list() {
        let input = "This is a paragraph.\n\n- This is a list item\n- This is another list item\n";
        let expected = vec![
            JdElement::Paragraph("This is a paragraph."),
            JdElement::UnorderedList(vec!["This is a list item", "This is another list item"]),
        ];
        let (_, output) = parse_jotdown(input).unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn unordered_list_and_paragraph() {
        let input = "- This is a list item\n- This is another list item\nThis is a paragraph.";
        let expected = vec![
            JdElement::UnorderedList(vec!["This is a list item", "This is another list item"]),
            JdElement::Paragraph("This is a paragraph."),
        ];
        let (_, output) = parse_jotdown(input).unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn paragraph_startswith_dash() {
        let input = "-This is not a list item";
        let expected = vec![JdElement::Paragraph("-This is not a list item")];
        let (_, output) = parse_jotdown(input).unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn bullet_in_header() {
        let input = "# This is a header with a bullet - in it\nThis is a paragraph.";
        let expected = vec![
            JdElement::TitleOrHeading(("This is a header with a bullet - in it", 1)),
            JdElement::Paragraph("This is a paragraph."),
        ];
        let (_, output) = parse_jotdown(input).unwrap();
        assert_eq!(output, expected);
    }
}
