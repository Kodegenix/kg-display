use super::*;

use nom::types::CompleteStr;

use std::str::FromStr;
use std::convert::TryFrom;


#[derive(Debug, Clone)]
pub struct Format {
    arg: Argument,
    spec: Option<FormatSpec>,
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
    pub fn parse(fmt_str: &str) -> Result<FormatString, ()> {
        match parse_format_string(fmt_str.into()) {
            Ok((_, fs)) => Ok(fs),
            Err(_) => Err(()),
        }
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


named!(parse_identifier<CompleteStr, CompleteStr>,
    recognize!(tuple!(take_while1!(|b: char| b == '_' || b.is_ascii_alphabetic()), take_while!(|b: char| b == '_' || b.is_ascii_alphanumeric())))
);

named!(parse_argument<CompleteStr, Argument>,
    alt!(
        map!(nom::digit, |i| Argument::Index(i.parse().unwrap())) |
        map!(parse_identifier, |n| Argument::Name(n.to_string()))
    )
);

named!(parse_fill<CompleteStr, char>,
    map!(take!(1), |s| s.chars().next().unwrap())
);

named!(parse_align<CompleteStr, Align>,
    map!(one_of!("<>^"), |a| Align::try_from(a).unwrap())
);

named!(parse_fill_align<CompleteStr, FillAlign>,
    alt!(
        map!(tuple!(parse_fill, parse_align), |(fill, align)| FillAlign {
            fill: Some(fill),
            align,
        }) |
        map!(parse_align, |align| FillAlign {
            fill: None,
            align,
        })
    )
);

named!(parse_sign<CompleteStr, Sign>,
    map!(one_of!("-+"), |s| Sign::try_from(s).unwrap())
);

named!(parse_alter<CompleteStr, bool>,
    map!(opt!(char!('#')), |o| o.is_some())
);

named!(parse_zero<CompleteStr, bool>,
    map!(opt!(char!('0')), |o| o.is_some())
);

named!(parse_count<CompleteStr, Count>,
    alt!(
        map!(terminated!(parse_argument, char!('$')), |a| Count::Argument(a)) |
        map!(nom::digit, |v| Count::Value(v.parse().unwrap()))
    )
);

named!(parse_precision<CompleteStr, Precision>,
    alt!(
        map!(char!('*'), |_| Precision::Star) |
        map!(parse_count, |c| Precision::Count(c))
    )
);

named!(parse_format_type<CompleteStr, FormatType>,
    map!(opt!(alt!(
        tag!("?") |
        tag!("x?") |
        tag!("X?") |
        tag!("o") |
        tag!("x") |
        tag!("X") |
        tag!("p") |
        tag!("b") |
        tag!("e") |
        tag!("E"))),
        |s| FormatType::from_str(&s.unwrap_or("".into())).unwrap())
);

named!(parse_format_spec<CompleteStr, FormatSpec>,
    do_parse!(
        char!(':') >>
        fill_align: opt!(parse_fill_align) >>
        sign: opt!(parse_sign) >>
        alter: parse_alter >>
        zero: parse_zero >>
        width: opt!(parse_count) >>
        precision: opt!(preceded!(char!('.'), parse_precision)) >>
        format_type: parse_format_type >>
        (FormatSpec {
            fill_align: fill_align,
            sign: sign,
            alter: alter,
            zero: zero,
            width: width,
            precision: precision,
            format_type: format_type,
        })
    )
);

named!(parse_format<CompleteStr, Format>,
    do_parse!(
        char!('{') >>
        arg: opt!(parse_argument) >>
        spec: opt!(parse_format_spec) >>
        char!('}') >>
        (Format {
            arg: arg.unwrap_or(Argument::Next),
            spec,
        })
    )
);

named!(parse_format_string<CompleteStr, FormatString>,
    map!(many0!(
        alt!(
            map!(is_not!("{}"), |s| FormatStringItem::Text(s.to_string())) |
            map!(tag!("{{"), |_| FormatStringItem::Escape('{')) |
            map!(tag!("}}"), |_| FormatStringItem::Escape('}')) |
            map!(parse_format, |f| FormatStringItem::Format(f))
        )
    ), |items| FormatString(items))
);


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_string() {
        let input = "aaa {{{}}} {username:#.2$?} dsd";
        let f = parse_format_string(input.into()).unwrap().1;
        let out = f.to_string();

        assert_eq!(input, out);
    }
}
