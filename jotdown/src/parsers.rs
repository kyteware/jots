use nom::{IResult, character::complete::{multispace0, not_line_ending, multispace1}, Err, error::Error, sequence::tuple, bytes::complete::take_while1};

/// Attempts to parse a paragraph.
/// Outputs the paragraph
pub fn parse_paragraph(input: &str) -> IResult<&str, &str> {
    let (input, paragraph) = not_line_ending1(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, paragraph))
}

pub fn parse_title_heading(input: &str) -> IResult<&str, (&str, u8)> {
    let (input, (num_hashtags, _, content)) = tuple((heading_tag, multispace1, not_line_ending))(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, (content, num_hashtags)))
}

fn heading_tag(input: &str) -> IResult<&str, u8> {
    let (input, hashtags) = take_while1(|c| c == '#')(input)?;
    let num_hashtags = hashtags.len() as u8;
    if num_hashtags > 6 {
        return Err(Err::Error(Error::new(input, nom::error::ErrorKind::TooLarge)));
    }
    Ok((input, num_hashtags))
}

pub fn not_line_ending1(input: &str) -> IResult<&str, &str> {
    let (input, paragraph) = not_line_ending(input)?;
    if paragraph.is_empty() {
        return Err(Err::Error(Error::new(input, nom::error::ErrorKind::NoneOf)));
    }
    Ok((input, paragraph))
}