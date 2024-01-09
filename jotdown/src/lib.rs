mod model;
mod parsers;
#[cfg(test)]
mod tests;

pub use model::JdElement;
use nom::{branch::alt, combinator::map, multi::many0, IResult};
use parsers::{parse_paragraph, parse_title_heading, parse_unordered_list, parse_ordered_list, parse_checklist};

pub fn parse_jotdown(input: &str) -> IResult<&str, Vec<JdElement>> {
    let (input, output) = many0(alt((
        map(parse_title_heading, JdElement::TitleOrHeading),
        map(parse_ordered_list, JdElement::OrderedList),
        map(parse_checklist, JdElement::Checklist),
        map(parse_unordered_list, JdElement::UnorderedList),
        map(parse_paragraph, JdElement::Paragraph),
    )))(input)?;

    Ok((input, output))
}