mod model;
mod parsers;

pub use model::JdElement;
use nom::{branch::alt, combinator::map, multi::many0, IResult};
use parsers::{parse_paragraph, parse_title_heading};

pub fn parse_jotdown(input: &str) -> IResult<&str, Vec<JdElement>> {
    let (input, output) = many0(alt((
        map(parse_title_heading, JdElement::TitleOrHeading),
        map(parse_paragraph, JdElement::Paragraph),
    )))(input)?;

    Ok((input, output))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paragraphs_2newlines_end() {
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
    fn test_paragraphs_noend() {
        let input = "This is a paragraph.\n\nThis is another paragraph.\n";
        let expected = vec![
            JdElement::Paragraph("This is a paragraph."),
            JdElement::Paragraph("This is another paragraph."),
        ];
        let (_, output) = parse_jotdown(input).unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_header() {
        let input = "## This is a header\nThis is a paragraph.";
        let expected = vec![
            JdElement::TitleOrHeading(("This is a header", 2)),
            JdElement::Paragraph("This is a paragraph."),
        ];
        let (_, output) = parse_jotdown(input).unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_titles() {
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
    fn test_paragraph_startswith_hashtag() {
        let input = "#This is not a header";
        let expected = vec![JdElement::Paragraph("#This is not a header")];
        let (_, output) = parse_jotdown(input).unwrap();
        assert_eq!(output, expected);
    }
}
