mod model;
mod parsers;
#[cfg(test)]
mod tests;
mod text_mods;

pub use model::{JdElement, JdText, JdTextMod};
use nom::{branch::alt, combinator::map, multi::many0, IResult};
use parsers::{
    empty_line, parse_checklist, parse_code_block, parse_embed, parse_ordered_list,
    parse_paragraph, parse_title_heading, parse_unordered_list,
};

pub fn parse_jotdown(input: &str) -> IResult<&str, Vec<JdElement>> {
    let (input, _) = many0(empty_line)(input)?;
    let (input, output) = many0(alt((
        map(parse_title_heading, JdElement::TitleOrHeading),
        map(parse_code_block, JdElement::CodeBlock),
        map(parse_ordered_list, JdElement::OrderedList),
        map(parse_checklist, JdElement::Checklist),
        map(parse_unordered_list, JdElement::UnorderedList),
        map(parse_embed, JdElement::Embed),
        map(parse_paragraph, JdElement::Paragraph),
    )))(input)?;

    Ok((input, output))
}
