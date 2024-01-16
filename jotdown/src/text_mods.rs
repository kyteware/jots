use std::slice;

use nom::{
    branch::alt,
    bytes::complete::{take_while1, take_until1},
    bytes::complete::{tag, take, take_while},
    combinator::map,
    sequence::delimited,
    IResult, character::complete::not_line_ending
};

use crate::{JdText, JdTextMod};

pub fn modded_text(input: &str) -> IResult<&str, JdText> {
    let (remaining, mut input) = not_line_ending(input)?;
    let mut acc = vec![];

    while let Ok((new_input, modded)) = alt((
        map(bold, JdTextMod::Bold),
        map(italic, JdTextMod::Italic),
        map(raw, JdTextMod::Raw),
        map(reference, JdTextMod::Reference),
        map(normal, JdTextMod::Normal),
    ))(input)
    {
        let last = acc.last_mut();
        match (last, modded) {
            (Some(JdTextMod::Normal(last)), JdTextMod::Normal(new)) => {
                *last = merge_two_strs(last, new);
            }
            (Some(JdTextMod::Bold(last)), JdTextMod::Bold(new)) => {
                *last = merge_two_strs(last, new);
            }
            (Some(JdTextMod::Italic(last)), JdTextMod::Italic(new)) => {
                *last = merge_two_strs(last, new);
            }
            (Some(JdTextMod::Raw(last)), JdTextMod::Raw(new)) => {
                *last = merge_two_strs(last, new);
            }
            (_, modded) => {
                acc.push(modded);
            }
        }
        input = new_input;
    }

    if acc.is_empty() {
        Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Not,
        )))
    } else {
        Ok((remaining, acc))
    }
}

fn bold(input: &str) -> IResult<&str, &str> {
    let (input, inner) = delimited(tag("**"), take_until1("**"), tag("**"))(input)?;
    // one_lined(inner)?;
    Ok((input, inner))
}

fn italic(input: &str) -> IResult<&str, &str> {
    let (input, inner) = delimited(tag("*"), take_until1("*"), tag("*"))(input)?;
    // one_lined(inner)?;
    Ok((input, inner))
}

fn raw(input: &str) -> IResult<&str, &str> {
    let (input, inner) = delimited(tag("`"), take_until1("`"), tag("`"))(input)?;
    // one_lined(inner)?;
    Ok((input, inner))
}

fn reference(input: &str) -> IResult<&str, (&str, &str)> {
    let (input, _) = tag("[")(input)?;
    let (input, descriptor) = take_while1(|c| c != ']')(input)?;
    // one_lined(descriptor)?;
    let (input, _) = tag("]")(input)?;
    let (input, _) = tag("(")(input)?;
    let (input, link) = take_while1(|c| c != ')')(input)?;
    // one_lined(link)?;
    let (input, _) = tag(")")(input)?;
    Ok((input, (descriptor, link)))
}

fn normal(input: &str) -> IResult<&str, &str> {
    let (new_input, non_delims) = take_while(|c| c != '*' && c != '`' && c != '[')(input)?;
    let res = if non_delims.is_empty() {
        dbg!("non_delims is empty");
        take(1usize)(input)?
    } else {
        (new_input, non_delims)
    };
    Ok(res)
}

// fn one_lined(input: &str) -> IResult<&str, &str> {
//     let (input, _) = not_line_ending(input)?;
//     if !input.is_empty() {
//         Err(Err::Error(Error::new(input, nom::error::ErrorKind::Not)))
//     } else {
//         Ok((input, ""))
//     }
// }

fn merge_two_strs<'a>(a: &'a str, b: &'a str) -> &'a str {
    let a_start = a.as_ptr();
    let b_start = b.as_ptr();
    if (b_start as usize) < (a_start as usize) {
        panic!("str b must begin after str a")
    }
    if b_start as usize - a_start as usize != a.len() {
        panic!("Cannot merge two strings that are not adjacent in memory");
    }
    let len = a.len() + b.len();
    unsafe {
        let s = slice::from_raw_parts(a_start, len);
        std::str::from_utf8_unchecked(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_strs_normal() {
        let og = "helloworld".to_string();
        let a = &og[1..5];
        let b = &og[5..];
        assert_eq!(merge_two_strs(a, b), "elloworld");
    }

    #[test]
    fn merge_strs_emojis() {
        let og = "hi✋🏿hello✋🏿".to_string();
        let a = &og[1..5];
        let b = &og[5..];
        assert_eq!(merge_two_strs(a, b), "i✋🏿hello✋🏿");
    }

    #[test]
    fn test_bold() {
        assert_eq!(bold("**bold**"), Ok(("", "bold")));
    }

    #[test]
    fn test_mods() {
        assert_eq!(
            modded_text("this is **bold** and *italic* and `raw` and [reference](link)\n\n"),
            Ok((
                "\n\n",
                vec![
                    JdTextMod::Normal("this is "),
                    JdTextMod::Bold("bold"),
                    JdTextMod::Normal(" and "),
                    JdTextMod::Italic("italic"),
                    JdTextMod::Normal(" and "),
                    JdTextMod::Raw("raw"),
                    JdTextMod::Normal(" and "),
                    JdTextMod::Reference(("reference", "link")),
                ]
            ))
        );
    }

    #[test]
    #[should_panic]
    fn test_nothing() {
        dbg!(modded_text("\n")).unwrap();
    }
}
