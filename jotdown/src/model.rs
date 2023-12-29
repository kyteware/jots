#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JdElement<'a> {
    /// Includes the number of #s, # is title, ##+ is a headings capped at 6
    TitleOrHeading((&'a str, u8)),
    List(Vec<&'a str>),
    Paragraph(&'a str)
}

