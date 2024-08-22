#[derive(Debug, PartialEq, Eq)]
pub enum Markdown<'a> {
    Bold(&'a str),
    Italic(&'a str),
    Underline(&'a str),
    Normal(&'a str),
    Link { text: &'a str, url: &'a str },
    InlineCode(&'a str),
    BlockCode { lang: &'a str, body: &'a str },
}

#[derive(PartialEq, Eq)]
enum Tag {
    Star,
    Star2,
    Underscore,
    Underscore2,
    Backtick,
    Backtick3,
    SquareBrackets,
    RoundBrackets,
    None,
}

pub fn parse<'a>(text: &'a str) -> Vec<Markdown<'a>> {
    let mut elements = Vec::new();
    let mut idx = 0;
    let mut start_idx = 0;
    let mut curr_tag = Tag::None;
    let mut prev_c_len = 1;
    while idx < text.len() {
        let c = text[idx..].chars().next().unwrap();
        let c_len = c.len_utf8();
        let c2 = if idx + c_len < text.len() {
            Some(text[idx + c_len..].chars().next().unwrap())
        } else {
            None
        };
        let prev_char = if idx < prev_c_len {
            None
        } else {
            Some(text[idx - prev_c_len..].chars().next().unwrap())
        };

        
        let c2_len = c2.map(char::len_utf8).unwrap_or(0);
        let chars = (c, c2);
        
        let mut found_tag = match chars {
            ('*', Some('*')) => Tag::Star2,
            ('_', Some('_')) => Tag::Underscore2,
            ('_', _) => Tag::Underscore,
            ('*', _) => Tag::Star,
            _ => Tag::None
        };
        if prev_char.is_some_and(|x| x == '\\') {
            // somehow remove the \ from the token stream
            // probably end the current tag and start a new one.
            elements.push(Markdown::Normal(&text[start_idx..idx - 1]));
            start_idx = idx;
            if found_tag == Tag::Star2 || found_tag == Tag::Underscore2 {
                idx += c2_len;
            }
            found_tag = Tag::None;
        }
        // let found_tag = if c == '*' && c2 == '*' {
        //     Tag::Star2
        // } else if c == '_' && c2 == '_' {
        //     Tag::Underscore2
        // } else if c == '*' {
        //     Tag::Star
        // } else if c == '_' {
        //     Tag::Underscore
        // } else if c == '`' {
        //     Tag::Backtick
        // } else {
        //     Tag::None
        // };

        if curr_tag != Tag::None && curr_tag == found_tag {
            let slice = &text[start_idx..idx];
            let md = match found_tag {
                Tag::Star | Tag::Underscore => Markdown::Italic(slice),
                Tag::Underscore2 => Markdown::Underline(slice),
                Tag::Star2 => Markdown::Bold(slice),
                Tag::Backtick => Markdown::InlineCode(slice),
                _ => unreachable!(),
            };
            elements.push(md);
            start_idx = idx + c_len + c2_len;
        }
        if curr_tag == Tag::None && found_tag != Tag::None {
            elements.push(Markdown::Normal(&text[start_idx..idx]));
            curr_tag = found_tag;
            start_idx = idx + c_len + c2_len;
            idx += c2_len; // skip the next char too, if it's not null
        }

        idx += c_len;
        prev_c_len = c_len;
    }
    let last_elem = Markdown::Normal(&text[start_idx..]);
    elements.push(last_elem);
    elements
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_text() {
        let input = "hello, 😀world✅!";
        let result = parse(input);
        assert_eq!(result, vec![Markdown::Normal(input)]);
    }
    #[test]
    fn normal_and_bold() {
        let input = "hello, 😀**world✅**!";
        let result = parse(input);
        assert_eq!(result, vec![Markdown::Normal("hello, 😀"), Markdown::Bold("world✅"), Markdown::Normal("!")]);
    }

    #[test]
    fn escapes() {
        let input = "hello, 😀\\**world✅**!";
        let result = parse(input);
        assert_eq!(result, vec![Markdown::Normal("hello, 😀"), Markdown::Normal("**world✅**!")]);
    }

}
