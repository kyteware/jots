use nom::{IResult, character::complete::multispace0, bytes::complete::take_until};

pub fn parse_paragraph(input: &str) -> IResult<&str, &str> {
    let (input, _) = multispace0(input)?;
    let (input, paragraph) = take_until("\n\n")(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, paragraph))
}