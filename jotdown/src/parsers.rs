use nom::{IResult, character::complete::{multispace0, not_line_ending}, Err, error::Error};

/// Attempts to parse a paragraph.
/// Outputs the paragraph
pub fn parse_paragraph(input: &str) -> IResult<&str, &str> {
    println!("trying to parse paragraph...");
    let (input, paragraph) = not_line_ending1(input)?;
    let (input, _) = multispace0(input)?;
    println!("parsed paragraph!");
    Ok((input, paragraph))
}

pub fn not_line_ending1(input: &str) -> IResult<&str, &str> {
    let (input, paragraph) = not_line_ending(input)?;
    if paragraph.is_empty() {
        return Err(Err::Error(Error::new(input, nom::error::ErrorKind::NoneOf)));
    }
    Ok((input, paragraph))
}