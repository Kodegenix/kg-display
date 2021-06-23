use std::convert::TryFrom;
use std::str::{FromStr, CharIndices};

#[derive(Debug, Clone)]
pub struct Format {
    arg: Argument,
    spec: Option<FormatSpec>,
}

impl Format {
    fn from_chars(chars: &mut CharIndices) -> Result<Format, String> {
        let mut f = Format {
            arg: Argument::Next,
            spec: None,
        };
        if let Some((_, '{')) = chars.next() {
            f.arg = Argument::from_chars(chars)?;
        } else {
            return Err(format!("format must start with '{{'"));
        }
        while let Some((i, c)) = chars.next() {
            match c {
                '}' => return Ok(f),
                ':' => f.spec = Some(FormatSpec::from_chars(chars)?),
                _ => return Err(format!("unexpected char '{}' at position {}", c, i)),
            }
        }
        Err(format!("unexpected end of input"))
    }
}

impl std::fmt::Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{")?;
        std::fmt::Display::fmt(&self.arg, f)?;
        if let Some(ref spec) = self.spec {
            write!(f, ":")?;
            std::fmt::Display::fmt(spec, f)?;
        }
        write!(f, "}}")
    }
}


#[derive(Debug, Clone)]
pub enum Argument {
    Next,
    Index(usize),
    Name(String),
}

impl Argument {
    fn from_chars(chars: &mut CharIndices) -> Result<Argument, String> {
        let off = chars.offset();
        let str = chars.as_str();
        let mut prev = chars.clone();
        let mut arg = Argument::Next;
        while let Some((_, c)) = chars.next() {
            match c {
                c if c.is_ascii_digit() => {
                    match arg {
                        Argument::Next => arg = Argument::Index(0),
                        _ => {},
                    }
                }
                c if c == '_' || c.is_ascii_alphabetic() => {
                    match arg {
                        Argument::Next => arg = Argument::Name(String::new()),
                        Argument::Index(_) => break,
                        Argument::Name(_) => {},
                    }
                }
                _ => break,
            }
            prev = chars.clone();
        }
        *chars = prev;
        match arg {
            Argument::Next => {},
            Argument::Index(ref mut index) => {
                *index = str[.. chars.offset() - off].parse::<usize>().map_err(|e| e.to_string())?;
            }
            Argument::Name(ref mut name) => {
                name.push_str(&str[.. chars.offset() - off]);
            }
        }
        return Ok(arg);
    }
}

impl std::fmt::Display for Argument {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Argument::Next => Ok(()),
            Argument::Index(index) => write!(f, "{}", index),
            Argument::Name(ref name) => write!(f, "{}", name),
        }
    }
}

impl Default for Argument {
    fn default() -> Self {
        Argument::Next
    }
}


#[derive(Debug, Clone, Copy)]
pub enum Align {
    Left,
    Center,
    Right,
}

impl std::fmt::Display for Align {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Align::Left => write!(f, "<"),
            Align::Center => write!(f, "^"),
            Align::Right => write!(f, ">"),
        }
    }
}

impl TryFrom<char> for Align {
    type Error = char;

    fn try_from(value: char) -> Result<Self, <Self as TryFrom<char>>::Error> {
        Ok(match value {
            '<' => Align::Left,
            '^' => Align::Center,
            '>' => Align::Right,
            _ => return Result::Err(value),
        })
    }
}


#[derive(Debug, Clone, Copy)]
pub struct FillAlign {
    fill: Option<char>,
    align: Align,
}

impl FillAlign {
    fn from_chars_opt(chars: &mut CharIndices) -> Result<Option<FillAlign>, String> {
        let prev = chars.clone();
        let mut step = 0;
        let mut fill = None;
        while let Some((_, c)) = chars.next() {
            match c {
                '^' | '<' | '>' => {
                    let align = Align::try_from(c).unwrap();
                    return Ok(Some(FillAlign {
                        fill,
                        align,
                    }));
                }
                _ if step == 0 => {
                    fill = Some(c);
                    step += 1;
                }
                _ => {
                    *chars = prev;
                    return Ok(None);
                }
            }
        }
        Ok(None)
    }
}

impl std::fmt::Display for FillAlign {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(c) = self.fill {
            write!(f, "{}", c)?;
        }
        write!(f, "{}", self.align)
    }
}


#[derive(Debug, Clone, Copy)]
pub enum Sign {
    Plus,
    Minus,
}

impl Sign {
    fn from_chars_opt(chars: &mut CharIndices) -> Result<Option<Sign>, String> {
        let prev = chars.clone();
        if let Some((_, c)) = chars.next() {
            return match Sign::try_from(c) {
                Ok(s) => Ok(Some(s)),
                Err(_) => {
                    *chars = prev;
                    Ok(None)
                }
            }
        }
        Ok(None)
    }
}

impl std::fmt::Display for Sign {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Sign::Plus => write!(f, "+"),
            Sign::Minus => write!(f, "-"),
        }
    }
}

impl TryFrom<char> for Sign {
    type Error = char;

    fn try_from(value: char) -> Result<Self, <Self as TryFrom<char>>::Error> {
        Ok(match value {
            '-' => Sign::Minus,
            '+' => Sign::Plus,
            _ => return Err(value),
        })
    }
}


#[derive(Debug, Clone)]
pub enum Precision {
    Count(Count),
    Star,
}

impl Precision {
    fn from_chars_opt(chars: &mut CharIndices) -> Result<Option<Precision>, String> {
        let prev = chars.clone();
        if let Some((_, c)) = chars.next() {
            return match c {
                '*' => Ok(Some(Precision::Star)),
                _ => {
                    *chars = prev;
                    match Count::from_chars_opt(chars)? {
                        Some(count) => Ok(Some(Precision::Count(count))),
                        None => Ok(None),
                    }
                }
            }
        }
        Ok(None)
    }
}

impl std::fmt::Display for Precision {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Precision::Count(ref count) => write!(f, ".{}", count),
            Precision::Star => write!(f, ".*"),
        }
    }
}


#[derive(Debug, Clone)]
pub enum Count {
    Argument(Argument),
    Value(usize),
}

impl Count {
    fn from_chars_opt(chars: &mut CharIndices) -> Result<Option<Count>, String> {
        let prev = chars.clone();
        let arg = Argument::from_chars(chars)?;

        let p = chars.clone();
        let arg_suffix = if let Some((_, '$')) = chars.next() {
            true
        } else {
            *chars = p;
            false
        };

        match arg {
            Argument::Next => Ok(None),
            Argument::Index(index) => if arg_suffix {
                Ok(Some(Count::Argument(arg)))
            } else {
                Ok(Some(Count::Value(index)))
            }
            Argument::Name(_) => if arg_suffix {
                Ok(Some(Count::Argument(arg)))
            } else {
                *chars = prev;
                Ok(None)
            }
        }
    }
}

impl std::fmt::Display for Count {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Count::Argument(ref arg) => write!(f, "{}$", arg),
            Count::Value(value) => write!(f, "{}", value),
        }
    }
}


#[derive(Debug, Clone)]
pub enum FormatType {
    Display,
    Debug,
    DebugLowerHex,
    DebugUpperHex,
    Octal,
    LowerHex,
    UpperHex,
    Pointer,
    Binary,
    LowerExp,
    UpperExp,
}

impl FormatType {
    fn from_chars(chars: &mut CharIndices) -> Result<FormatType, String> {
        let mut prev = chars.clone();
        let str = chars.as_str();
        let off = chars.offset();
        while let Some((_, c)) = chars.next() {
            if c == '}' {
                *chars = prev;
                break;
            } else {
                prev = chars.clone();
            }
        }
        let s = &str[.. chars.offset() - off];
        FormatType::from_str(s).map_err(|_| format!("unrecognized value type: '{}'", s))
    }
}

impl std::fmt::Display for FormatType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            FormatType::Display => Ok(()),
            FormatType::Debug => write!(f, "?"),
            FormatType::DebugLowerHex => write!(f, "x?"),
            FormatType::DebugUpperHex => write!(f, "X?"),
            FormatType::Octal => write!(f, "o"),
            FormatType::LowerHex => write!(f, "x"),
            FormatType::UpperHex => write!(f, "X"),
            FormatType::Pointer => write!(f, "p"),
            FormatType::Binary => write!(f, "b"),
            FormatType::LowerExp => write!(f, "e"),
            FormatType::UpperExp => write!(f, "E"),
        }
    }
}

impl FromStr for FormatType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        Ok(match s {
            "" => FormatType::Display,
            "?" => FormatType::Debug,
            "x?" => FormatType::DebugLowerHex,
            "X?" => FormatType::DebugUpperHex,
            "o" => FormatType::Octal,
            "x" => FormatType::LowerHex,
            "X" => FormatType::UpperHex,
            "p" => FormatType::Pointer,
            "b" => FormatType::Binary,
            "e" => FormatType::LowerExp,
            "E" => FormatType::UpperExp,
            _ => return Err(()),
        })
    }
}


#[derive(Debug, Clone)]
pub struct FormatSpec {
    fill_align: Option<FillAlign>,
    sign: Option<Sign>,
    alter: bool,
    zero: bool,
    width: Option<Count>,
    precision: Option<Precision>,
    format_type: FormatType,
}

impl FormatSpec {
    fn from_chars(chars: &mut CharIndices) -> Result<FormatSpec, String> {
        let mut prev = chars.clone();
        let mut spec = FormatSpec {
            fill_align: FillAlign::from_chars_opt(chars)?,
            sign: Sign::from_chars_opt(chars)?,
            alter: false,
            zero: false,
            width: None,
            precision: None,
            format_type: FormatType::Display,
        };
        let mut step = 0;
        while let Some((i, c)) = chars.next() {
            match c {
                '}' => {
                    *chars = prev;
                    return Ok(spec);
                }
                '#' if step == 0 => {
                    spec.alter = true;
                    step += 1;
                }
                '0' if step < 2 => {
                    spec.zero = true;
                    step = 2;
                }
                _ if step < 3 => {
                    *chars = prev;
                    spec.width = Count::from_chars_opt(chars)?;
                    step = 3;
                }
                '.' if step < 4 => {
                    spec.precision = Precision::from_chars_opt(chars)?;
                    step = 4;
                }
                _ if step < 5 => {
                    *chars = prev;
                    spec.format_type = FormatType::from_chars(chars)?;
                    step = 5;
                }
                _ => return Err(format!("unexpected char '{}' at position {}", c, i))
            }
            prev = chars.clone();
        }
        Err(format!("unexpected end of input"))
    }
}

impl std::fmt::Display for FormatSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(a) = self.fill_align {
            write!(f, "{}", a)?;
        }
        if let Some(s) = self.sign {
            write!(f, "{}", s)?;
        }
        if self.alter {
            write!(f, "#")?;
        }
        if self.zero {
            write!(f, "0")?;
        }
        if let Some(ref w) = self.width {
            write!(f, "{}", w)?;
        }
        if let Some(ref p) = self.precision {
            write!(f, "{}", p)?;
        }
        write!(f, "{}", self.format_type)
    }
}


#[derive(Debug, Clone)]
pub enum FormatStringItem {
    Text(String),
    Escape(char),
    Format(Format),
}

impl std::fmt::Display for FormatStringItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            FormatStringItem::Text(ref s) => write!(f, "{}", s),
            FormatStringItem::Escape(c) => write!(f, "{}{}", c, c),
            FormatStringItem::Format(ref fmt) => write!(f, "{}", fmt),
        }
    }
}


#[derive(Debug, Clone)]
pub struct FormatString(Vec<FormatStringItem>);

impl FormatString {
    pub fn parse(fmt_str: &str) -> Result<FormatString, String> {
        let mut chars = fmt_str.char_indices();
        Self::from_chars(&mut chars)
    }

    fn from_chars(chars: &mut CharIndices) -> Result<FormatString, String> {
        let mut items = Vec::new();
        let mut s = String::new();
        let mut prev = chars.clone();
        while let Some((i, c)) = chars.next() {
            match c {
                '{' => {
                    if !s.is_empty() {
                        items.push(FormatStringItem::Text(s));
                        s = String::new();
                    }
                    if chars.as_str().starts_with('{') {
                        chars.next();
                        items.push(FormatStringItem::Escape('{'));
                    } else {
                        *chars = prev;
                        let f = Format::from_chars(chars)?;
                        items.push(FormatStringItem::Format(f));
                    }
                }
                '}' => {
                    if chars.as_str().starts_with('}') {
                        if !s.is_empty() {
                            items.push(FormatStringItem::Text(s));
                            s = String::new();
                        }
                        items.push(FormatStringItem::Escape('}'));
                        chars.next();
                    } else {
                        return Err(format!("unescaped '{{' at position {}", i));
                    }
                }
                _ => s.push(c),
            }
            prev = chars.clone();
        }
        if !s.is_empty() {
            items.push(FormatStringItem::Text(s));
        }
        Ok(FormatString(items))
    }

    pub fn items(&self) -> &[FormatStringItem] {
        &self.0
    }

    pub fn each_argument<F>(&self, mut f: F) where F: FnMut(&Argument) -> bool {
        for item in self.0.iter() {
            if let FormatStringItem::Format(ref fmt) = item {
                if !f(&fmt.arg) {
                    break;
                }
                if let Some(ref spec) = fmt.spec {
                    if let Some(ref width) = spec.width {
                        if let Count::Argument(ref arg) = width {
                            if !f(arg) {
                                break;
                            }
                        }
                    }
                    if let Some(ref prec) = spec.precision {
                        if let Precision::Count(Count::Argument(ref arg)) = prec {
                            if !f(arg) {
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
}

impl std::fmt::Display for FormatString {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for i in self.0.iter() {
            write!(f, "{}", i)?;
        }
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_string() {
        let input = "aaa {{}} {{{}}} {username:.^#02X} {123} {:?} dsd";
        let f = FormatString::parse(input).unwrap();
        let out = f.to_string();
        assert_eq!(input, out);
    }
}
