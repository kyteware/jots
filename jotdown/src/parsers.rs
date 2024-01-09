//! Parsers should follow the following rules:
//! - Consume all leading whitespace
//! - Consume no trailing whitespace

use nom::{
    bytes::complete::{take_while1, tag},
    character::complete::{multispace0, multispace1, not_line_ending, line_ending, u64 as nom_u64, space0},
    error::Error,
    sequence::{tuple, terminated},
    Err, IResult, multi::{many1, many0}, combinator::opt,
};

pub fn parse_paragraph(input: &str) -> IResult<&str, &str> {
    let (input, _) = many0(empty_line)(input)?;
    let (input, paragraph) = not_line_ending1(input)?;
    Ok((input, paragraph))
}

pub fn parse_title_heading(input: &str) -> IResult<&str, (&str, u8)> {
    let (input, _) = many0(empty_line)(input)?;
    let (input, (num_hashtags, _, content)) =
        tuple((heading_tag, multispace1, not_line_ending))(input)?;
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
    let (input, _) = many0(empty_line)(input)?;
    let (input, list) = many1(parse_unordered_list_entry)(input)?;
    Ok((input, list))
}

fn parse_unordered_list_entry(input: &str) -> IResult<&str, &str> {
    let (input, (_, content, _)) =
        tuple((tag("- "), not_line_ending, opt(line_ending)))(input)?;
    Ok((input, content))
}

pub fn parse_ordered_list(input: &str) -> IResult<&str, Vec<&str>> {
    let (mut input, _) = many0(empty_line)(input)?;
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
    dbg!(Ok((input, list)))
}

fn parse_ordered_list_entry(input: &str) -> IResult<&str, (u64, &str)> {
    let (input, (num, _, content, _)) =
        tuple((nom_u64, tag(". "), not_line_ending, opt(line_ending)))(input)?;
    Ok((input, (num, content)))
}

fn not_line_ending1(input: &str) -> IResult<&str, &str> {
    let (input, paragraph) = not_line_ending(input)?;
    if paragraph.is_empty() {
        return Err(Err::Error(Error::new(input, nom::error::ErrorKind::NoneOf)));
    }
    Ok((input, paragraph))
}

fn empty_line(input: &str) -> IResult<&str, &str> {
    terminated(space0, line_ending)(input)
}