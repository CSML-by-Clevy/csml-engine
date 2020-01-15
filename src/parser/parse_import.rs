use crate::parser::{ast::*, parse_comments::comment, parse_ident::*, tokens::*, tools::*, context::*};
use nom::{bytes::complete::tag, combinator::opt, error::ParseError, sequence::preceded, *};

fn step_namet<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Identifier, E> {
    let (s, _) = preceded(comment, parse_import_step)(s)?;
    let (s, name) = parse_ident(s)?;
    Ok((s, name))
}

fn as_name<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Identifier, E> {
    let (s, _) = preceded(comment, tag(AS))(s)?;
    let (s, name) = parse_ident(s)?;
    Ok((s, name))
}

fn file_path<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Identifier, E> {
    let (s, _) = preceded(comment, tag(FROMEFILE))(s)?;
    let (s, name) = parse_ident(s)?;
    Ok((s, name))
}

pub fn parse_import_opt<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    let (s, step_name) = step_namet(s)?;
    let (s, as_name) = opt(as_name)(s)?;
    let (s, file_path) = opt(file_path)(s)?;
    Ok((
        s,
        Expr::ObjectExpr(ObjectType::Import {
            step_name,
            as_name,
            file_path,
        }),
    ))
}

pub fn parse_import<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, (Expr, InstructionInfo), E> {
    let (s, _) = tag(IMPORT)(s)?;
    let (s, name) = parse_import_opt(s)?;

    let instruction_info = InstructionInfo{index:Context::get_index(), total:0};

    Context::inc_index();

    Ok((s, (name, instruction_info)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::error::ErrorKind;

    pub fn test_import<'a>(s: Span<'a>) -> IResult<Span<'a>, (Expr, InstructionInfo)> {
        let var = parse_import(s);
        if let Ok((s, v)) = var {
            if s.fragment.len() != 0 {
                Err(Err::Error((s, ErrorKind::Tag)))
            } else {
                Ok((s, v))
            }
        } else {
            var
        }
    }

    #[test]
    fn ok_step_import() {
        let string = Span::new("import step hola");
        match test_import(string) {
            Ok(..) => {}
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn ok_step_import_as() {
        let string = Span::new("import step hola as test");
        match test_import(string) {
            Ok(..) => {}
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn ok_step_import_as_from_file() {
        let string = Span::new("import step hola as test FromFile filetest");
        match test_import(string) {
            Ok(..) => {}
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn err_step_import1() {
        let string = Span::new("import hola");
        match test_import(string) {
            Ok(..) => panic!("need to fail"),
            Err(..) => {}
        }
    }

    #[test]
    fn err_step_import2() {
        let string = Span::new("import step");
        match test_import(string) {
            Ok(..) => panic!("need to fail"),
            Err(..) => {}
        }
    }

    #[test]
    fn err_step_import_as() {
        let string = Span::new("import step hola as");
        match test_import(string) {
            Ok(..) => panic!("need to fail"),
            Err(..) => {}
        }
    }

    #[test]
    fn err_step_import_as_from_file() {
        let string = Span::new("import step hola as");
        match test_import(string) {
            Ok(..) => panic!("need to fail"),
            Err(..) => {}
        }
    }
}
