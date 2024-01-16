mod model;
mod parsers;
#[cfg(test)]
mod tests;
mod text_mods;
mod unparsers;

pub use model::{JdElement, JdText, JdTextMod};
use nom::{branch::alt, combinator::map, multi::many0, IResult};
use parsers::{
    empty_line, parse_checklist, parse_code_block, parse_embed, parse_ordered_list,
    parse_paragraph, parse_title_heading, parse_unordered_list,
};
use unparsers::{unparse_code_block, unparse_ordered_list, unparse_checklist, unparse_unordered_list, unparse_embed, unparse_paragraph};

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

pub fn unparse_jotdown(input: &[JdElement]) -> String {
    let mut output = String::new();
    for (i, element) in input.iter().enumerate() {
        match element {
            JdElement::TitleOrHeading((text, level)) => {
                unparsers::unparse_title_heading(&mut output, text, *level)
            }
            JdElement::CodeBlock((code, lang)) => unparse_code_block(&mut output, *lang, code),
            JdElement::OrderedList(list) => unparse_ordered_list(&mut output, list),
            JdElement::Checklist(list) => unparse_checklist(&mut output, list),
            JdElement::UnorderedList(list) => unparse_unordered_list(&mut output, list),
            JdElement::Embed(embed) => unparse_embed(&mut output, embed),
            JdElement::Paragraph(text) => unparse_paragraph(&mut output, text),
        }

        if i < input.len() - 1 {
            output.push('\n');
        }
    }
    output
}
