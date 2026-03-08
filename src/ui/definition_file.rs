use std::io::{Read, Seek};

use winnow::Parser;
use winnow::ascii::{
    dec_uint, escaped, hex_uint, line_ending, multispace0, space0, space1, till_line_ending,
};
use winnow::combinator::{
    alt, backtrack_err, cut_err, delimited, dispatch, eof, fail, opt, peek, preceded, repeat,
    repeat_till, seq, terminated, trace,
};
use winnow::error::{AddContext, ParserError, StrContext, StrContextValue};
use winnow::token::{any, none_of, one_of, take_till};

use crate::types::{Mat4, MotionVectors, Vec4};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct DefinitionFile {
    pub katamari: Vec<Field>,
    pub prince: Vec<Field>,
    pub thing: Vec<Field>,
    pub camera: Vec<Field>,
    pub camera_transform: Vec<Field>,
    pub global: Vec<Field>,
}

impl DefinitionFile {
    pub fn load(mut f: &std::fs::File) -> Result<Self, String> {
        let mut text = String::new();
        f.rewind().map_err(|e| e.to_string())?;
        f.read_to_string(&mut text).map_err(|e| e.to_string())?;
        let this = document.parse(&text).map_err(|e| {
            println!("{e}");
            println!("{e:?}");
            e.to_string()
        })?;
        Ok(this)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldType {
    Float,
    Vec4,
    Mat4,
    Bool,
    Int8 { signed: bool },
    Int16 { signed: bool },
    Int32 { signed: bool },
    Int64 { signed: bool },
    Address,
    MotionVectors,
    Array(Box<Self>, usize),
}

impl FieldType {
    pub fn size(&self) -> usize {
        use std::mem::size_of;
        match self {
            Self::Int8 { .. } | Self::Bool => 1,
            Self::Int16 { .. } => 2,
            Self::Int32 { .. } | Self::Float => 4,
            Self::Int64 { .. } | Self::Address => 8,
            Self::Vec4 => size_of::<Vec4>(), // 0x10 / 16
            Self::Mat4 => size_of::<Mat4>(), // 0x40 / 64
            Self::MotionVectors => size_of::<MotionVectors>(),
            Self::Array(inner, count) => inner.size() * count,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldPos {
    pub base: isize,
    pub offset: Option<isize>,
}

#[allow(dead_code)]
impl FieldPos {
    pub const fn new(base: isize) -> Self {
        Self { base, offset: None }
    }

    pub const fn with_offset(self, offset: isize) -> Self {
        Self {
            base: self.base,
            offset: Some(offset),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field {
    pub position: FieldPos,
    pub field_type: FieldType,
    pub name: Option<String>,
}

macro_rules! parsers {
    ($(
        $vis:vis $name:ident $(<$lt:lifetime>)? : $ty:ty = $body:expr;
    )*) => {$(
        $vis fn $name $(<$lt>)? (input: &mut & $($lt)? str) -> winnow::ModalResult<$ty> {
            $body.parse_next(input)
        }
    )*}
}

fn string_literal<'i, E>(quote: char) -> impl winnow::ModalParser<&'i str, String, E>
where
    E: ParserError<&'i str> + AddContext<&'i str, StrContext>,
{
    let control = '\\';
    let esc = [control, quote];
    cut_err(delimited(
        quote,
        escaped(none_of((esc, '\n')), control, one_of(esc)),
        quote.context(StrContext::Expected(StrContextValue::CharLiteral(quote))),
    ))
}

fn integer_type<'i, E>(signed: bool) -> impl winnow::ModalParser<&'i str, FieldType, E>
where
    E: ParserError<&'i str>,
{
    alt((
        "8".value(FieldType::Int8 { signed }),
        "16".value(FieldType::Int16 { signed }),
        "32".value(FieldType::Int32 { signed }),
        "64".value(FieldType::Int64 { signed }),
    ))
}

parsers! {
    field_type: FieldType = dispatch! {any;
        'f' => "32".value(FieldType::Float),
        'v' | 'V' => "ec4".value(FieldType::Vec4),
        'm' | 'M' => "at4".value(FieldType::Mat4),
        'b' => "ool".value(FieldType::Bool),
        'i' => integer_type(true),
        'u' => integer_type(false),
        'a' => "ddr".value(FieldType::Address),
        'p' => "tr".value(FieldType::Address),
        '@' => alt((
            "motion_vectors".value(FieldType::MotionVectors),
        )),
        '[' => seq!(FieldType::Array(
            field_type.map(Box::new),
            _: ';',
            _: space0,
            dec_uint,
            _: ']',
        )),
        _ => fail,
    }.context(StrContext::Label("field type"));

    name: String = dispatch!{peek(any);
        '\'' => string_literal('\''),
        '\"' => string_literal('"'),
        // This variation still has messed up error reporting. Whatever.
        _ => take_till(1.., [';', ' ', '\r', '\n']).map(String::from),
    }.context(StrContext::Label("field name"));

    field_pos: FieldPos = seq!{FieldPos{
        _: 'x',
        base: hex_uint.map(|n: u64| n as isize),
        offset: opt(preceded('+', hex_uint).map(|n: u64| n as isize)),
    }};

    field: Field = seq!{Field{
        position: terminated(field_pos, space1).context(StrContext::Label("field address")),
        field_type: field_type,
        name: opt(preceded(space1, name)),
    }};

    comment<'a>: &'a str = trace("comment", preceded(';', till_line_ending));

    line: Field = trace("line", delimited(
        space0,
        cut_err(field),
        (
            space0,
            opt(comment),
            alt((line_ending, eof))
        ),
    )).context(StrContext::Label("line"));

    blank_line: () = alt((
        (space0, comment, alt((line_ending, eof))).void(),
        (space0, line_ending).void(),
        space1.void(),
    ));
    junk: () = trace("junk", repeat(0.., blank_line.void()));

    header<'a>: &'a str = delimited(
        (multispace0, '['),
        take_till(1.., ']'),
        ']',
    ).context(StrContext::Label("header"));

    body: Vec<Field> = repeat_till(
        0..,
        terminated(line, junk),
        peek(alt(("[", eof))),
    ).map(|(acc, _)| acc);

    section<'a>: (&'a str, Vec<Field>) = (terminated(header, (line_ending, junk)), body);

    document: DefinitionFile = repeat(0.., section).verify_fold(
        DefinitionFile::default,
        |mut doc, section| {
            let (name, lines) = section;
            let target = match name {
                "katamari" => &mut doc.katamari,
                "prince" => &mut doc.prince,
                "thing" => &mut doc.thing,
                "camera" => &mut doc.camera,
                "camera_transform" => &mut doc.camera_transform,
                "global" => &mut doc.global,
                _ => return None,
            };
            target.extend(lines);
            Some(doc)
        },
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_line() {
        let input = "x40 [vec4; 4] foo ;bar";
        let parsed = line.parse(input).unwrap();
        assert_eq!(
            parsed,
            Field {
                position: FieldPos::new(0x40),
                field_type: FieldType::Array(Box::new(FieldType::Vec4), 4),
                name: Some("foo".into()),
            },
        );
    }

    #[test]
    fn parse_section() {
        let input = r#"
            [katamari]
            x40 vec4 position
            
            x50 [mat4; 8] 'the \'guys\'' ; why are there so many
            ; here be dragons
            x254 f32
            x25c addr ; what does he point to

            
        "#;
        let parsed = section
            .parse(input)
            .inspect_err(|e| println!("{e}"))
            .unwrap();
        let expected = vec![
            Field {
                position: FieldPos::new(0x40),
                field_type: FieldType::Vec4,
                name: Some("position".into()),
            },
            Field {
                position: FieldPos::new(0x50),
                field_type: FieldType::Array(FieldType::Mat4.into(), 8),
                name: Some("the 'guys'".into()),
            },
            Field {
                position: FieldPos::new(0x254),
                field_type: FieldType::Float,
                name: None,
            },
            Field {
                position: FieldPos::new(0x25c),
                field_type: FieldType::Address,
                name: None,
            },
        ];
        assert_eq!(parsed, ("katamari", expected));
    }

    #[test]
    fn parse_document() {
        let input = r#"
            [katamari]
            x40 vec4 position
            
            x50 [mat4; 8] 'the \'guys\'' ; why are there so many
            ; here be dragons

            [prince]
            x254 f32
            x25c addr ; what does he point to
        "#;
        let parsed = document
            .parse(input)
            .inspect_err(|e| println!("{e}"))
            .unwrap();
        let expected = DefinitionFile {
            katamari: vec![
                Field {
                    position: FieldPos::new(0x40),
                    field_type: FieldType::Vec4,
                    name: Some("position".into()),
                },
                Field {
                    position: FieldPos::new(0x50),
                    field_type: FieldType::Array(FieldType::Mat4.into(), 8),
                    name: Some("the 'guys'".into()),
                },
            ],
            prince: vec![
                Field {
                    position: FieldPos::new(0x254),
                    field_type: FieldType::Float,
                    name: None,
                },
                Field {
                    position: FieldPos::new(0x25c),
                    field_type: FieldType::Address,
                    name: None,
                },
            ],
            ..Default::default()
        };
        assert_eq!(parsed, expected);
    }

    #[test]
    fn field_offset_err() {
        let mut input = r#"
            [prince]
            ; foo

            [global]
            x153150 ptr
            x7a050 f32
            x7a054 f32
            x7a058 f32
            xd35380 ptr
            x10EB18 u32 'CLIMB SUSTAIN LIMIT'
            ×10eac8 i16 'dust cooldown'

            [camera_transform]
            ; foo
        "#;
        let err = document.parse(&mut input).unwrap_err();
        assert_eq!(err.offset(), err.input().find('×').unwrap());
        assert!(
            err.inner()
                .context()
                .any(|c| *c == StrContext::Label("field address"))
        );
    }

    #[test]
    fn field_name_err() {
        let mut input = r#"
            [prince]
            ; foo
            x10 u8 foo

            [thing]
            x3b8+8 ptr "full machine
            x90 vec4 position
            xa0 vec4 rotation

            [camera_transform]
            ; foo
        "#;
        let err = document.parse(&mut input).unwrap_err();
        let msg = err.to_string();
        println!("{msg}");
        println!("{err:?}");
        assert!(msg.contains("invalid field name"));
        assert!(msg.contains("expected `\"`"));
    }
}
