#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JdElement<'a> {
    /// Includes the number of #s, # is title, ##+ is a headings capped at 6
    TitleOrHeading((&'a str, u8)),
    UnorderedList(Vec<&'a str>),
    OrderedList(Vec<&'a str>),
    Paragraph(&'a str),
}
