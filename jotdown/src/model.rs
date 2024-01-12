#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JdElement<'a> {
    /// Includes the number of #s, # is title, ##+ is a headings capped at 6
    TitleOrHeading((JdText<'a>, u8)),
    CodeBlock((&'a str, Option<&'a str>)),
    Checklist(Vec<(JdText<'a>, bool)>),
    UnorderedList(Vec<JdText<'a>>),
    OrderedList(Vec<JdText<'a>>),
    Embed(&'a str),
    Paragraph(JdText<'a>),
}

pub type JdText<'a> = Vec<JdTextMod<'a>>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JdTextMod<'a> {
    Normal(&'a str),
    Bold(&'a str),
    Italic(&'a str),
    Raw(&'a str),
    Reference((&'a str, &'a str)),
}
