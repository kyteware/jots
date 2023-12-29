mod model;
mod parsers;

use nom::{IResult, multi::many0, branch::alt, combinator::map};
pub use model::JdElement;
use parsers::parse_paragraph;

pub fn parse_jotdown(input: &str) -> IResult<&str, Vec<JdElement>> {
    let (input, output) = many0(
        alt(
            (
                map(parse_paragraph, JdElement::Paragraph),
            )
        )
    )(input)?;

    Ok((input, output))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paragraphs_2newlines_end() {
        let input = "This is a paragraph.\n\nThis is another paragraph.\n\n";
        let expected = vec![
            JdElement::Paragraph("This is a paragraph."),
            JdElement::Paragraph("This is another paragraph.")
        ];
        let (input, output) = parse_jotdown(input).unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_paragraphs_2newlines_noend() {
        let input = "This is a paragraph.\n\nThis is another paragraph.\n";
        let expected = vec![
            JdElement::Paragraph("This is a paragraph."),
            JdElement::Paragraph("This is another paragraph.")
        ];
        let (input, output) = parse_jotdown(input).unwrap();
        assert_eq!(output, expected);
    }
}