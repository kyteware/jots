//! Parsers should consume all trailing empty lines

use nom::{
    bytes::complete::{take_while1, tag},
    character::{complete::{multispace1, not_line_ending, line_ending, u64 as nom_u64, space0, space1}, is_space},
    error::Error,
    sequence::{tuple, terminated, delimited},
    Err, IResult, multi::{many1, many0}, combinator::{opt, eof}, branch::alt,
};

pub fn parse_paragraph(input: &str) -> IResult<&str, &str> {
    let (input, paragraph) = not_line_ending1(input)?;
    let (input, _) = many0(empty_line)(input)?;
    Ok((input, paragraph))
}

pub fn parse_title_heading(input: &str) -> IResult<&str, (&str, u8)> {
    let (input, (num_hashtags, _, content)) =
        tuple((heading_tag, multispace1, not_line_ending))(input)?;
    let (input, _) = many0(empty_line)(input)?;
    Ok((input, (content, num_hashtags)))
}

fn heading_tag(input: &str) -> IResult<&str, u8> {
    let (input, hashtags) = take_while1(|c| c == '#')(input)?;
    let num_hashtags = hashtags.len() as u8;
    if num_hashtags > 6 {
        return Err(Err::Error(Error::new(
            input,
            nom::error::ErrorKind::TooLarge,
        )));
    }
    Ok((input, num_hashtags))
}

pub fn parse_unordered_list(input: &str) -> IResult<&str, Vec<&str>> {
    let (input, list) = many1(parse_unordered_list_entry)(input)?;
    let (input, _) = many0(empty_line)(input)?;
    Ok((input, list))
}

fn parse_unordered_list_entry(input: &str) -> IResult<&str, &str> {
    if parse_checklist_entry(input).is_ok() {
        return Err(Err::Error(Error::new(
            input,
            nom::error::ErrorKind::IsA,
        )));
    }
    let (input, (_, content, _)) =
        tuple((tag("- "), not_line_ending, opt(line_ending)))(input)?;
    Ok((input, content))
}

pub fn parse_ordered_list(mut input: &str) -> IResult<&str, Vec<&str>> {
    let mut list = Vec::new();
    let mut expected_num = 1;
    while let Ok((adj_input, (num, entry))) = parse_ordered_list_entry(input) {
        if num != expected_num {
            break;
        }
        list.push(entry);
        input = adj_input;
        expected_num += 1;
    }
    if list.is_empty() {
        return Err(Err::Error(Error::new(
            input,
            nom::error::ErrorKind::Many1,
        )));
    }
    let (input, _) = many0(empty_line)(input)?;
    Ok((input, list))
}

fn parse_ordered_list_entry(input: &str) -> IResult<&str, (u64, &str)> {
    let (input, (num, _, content, _)) =
        tuple((nom_u64, tag(". "), not_line_ending, opt(line_ending)))(input)?;
    Ok((input, (num, content)))
}

pub fn parse_checklist(input: &str) -> IResult<&str, Vec<(&str, bool)>> {
    let (input, list) = many1(parse_checklist_entry)(input)?;
    let (input, _) = many0(empty_line)(input)?;
    Ok((input, list))
}

fn parse_checklist_entry(input: &str) -> IResult<&str, (&str, bool)> {
    let (input, (_, check, _, content, _)) =
        tuple((tag("- ["), alt((tag("x"), tag(" "))), tag("] "), not_line_ending, opt(line_ending)))(input)?;
    let checked = check == "x";
    Ok((input, (content, checked)))
}

// pub fn parse_reference(input: &str) -> IResult<&str, (&str, &str)> {
//     let (input, _) = many0(empty_line)(input)?;
//     tuple((delimited(tag("["), take_while1(|c| !is_space(c as u8)), tag("]")), delimited(tag("("), not_line_ending1, tag(")"))))(input)
// }

pub fn parse_embed(input: &str) -> IResult<&str, &str> {
    dbg!(input);
    let (input, embed) = delimited(tag("["), take_while1(|c| !is_space(dbg!(c) as u8) && c != ']'), tag("]"))(input)?;
    let (input, _) = many0(empty_line)(input)?;
    Ok((input, embed))
}

fn not_line_ending1(input: &str) -> IResult<&str, &str> {
    let (input, paragraph) = not_line_ending(input)?;
    if paragraph.is_empty() {
        return Err(Err::Error(Error::new(input, nom::error::ErrorKind::NoneOf)));
    }
    Ok((input, paragraph))
}

pub fn empty_line(input: &str) -> IResult<&str, &str> {
    alt((terminated(space0, line_ending), terminated(space1, eof)))(input)
}

#[cfg(test)]
mod basic_tests {
    use super::*;

    #[test]
    fn empty_lines_real() {
        assert_eq!(empty_line("\n"), Ok(("", "")));
        assert_eq!(empty_line(" \n"), Ok(("", " ")));
    }
}