use regex::Regex;

#[deriving(Show)]
pub enum Element<'s> {
    Header(&'s str, uint),
    Break,
    Paragraph(&'s str)
}

static SPLIT : Regex = regex!(r"\n\n");
static ATX_HEADER : Regex = regex!(r"(?P<level>#{1,6})\s(?P<text>[^#]*)#*");
static SETEXT_HEADER_1 : Regex = regex!(r"(?P<text>.+)\n===+");
static SETEXT_HEADER_2 : Regex = regex!(r"(?P<text>.+)\n---+");
static BREAK : Regex = regex!(r"  ");

pub fn parse (md : &str) -> Vec<Element> {
    let mut split = SPLIT.split(md);
    let mut tokens = vec![];
    for text in split{
        tokens.push(parse_segment(text));
    }
    tokens
}

fn parse_segment (text : &str) -> Element{
    if ATX_HEADER.is_match(text){
        let caps = ATX_HEADER.captures(text).unwrap();
        return Header (
            caps.name("text"),
            caps.name("level").len()
            );
    }else if SETEXT_HEADER_1.is_match(text){
        let caps = SETEXT_HEADER_1.captures(text).unwrap();
        return Header (caps.name("text"), 1);
    }else if SETEXT_HEADER_2.is_match(text){
        let caps = SETEXT_HEADER_2.captures(text).unwrap();
        return Header (caps.name("text"), 2);
    }else if BREAK.is_match(text){
        return Break;
    }else{
        return Paragraph(text);
    }
}

