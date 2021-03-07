use super::Value;
use crate::output::Formatted;
use crate::value::{ListSeparator, Operator, Quotes};
use std::fmt::{self, Display, Write};

impl<'a> Display for Formatted<'a, Value> {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match *self.value {
            Value::Bang(ref s) => write!(out, "!{}", s),
            Value::Literal(ref s, ref q) => match *q {
                Quotes::None => write!(out, "{}", s),
                Quotes::Double => {
                    if s.contains('"') && !s.contains('\'') {
                        write_sq(out, s)
                    } else {
                        write_dq(out, s)
                    }
                }
                Quotes::Single => {
                    if !s.contains('"') || s.contains('\'') {
                        write_dq(out, s)
                    } else {
                        write_sq(out, s)
                    }
                }
            },
            Value::Function(ref n, ref _f) => {
                let name = n
                    .chars()
                    .flat_map(|c| match c {
                        '"' => vec!['\\', '"'],
                        c => vec![c],
                    })
                    .collect::<String>();
                write!(out, "get-function(\"{}\")", name)
            }
            Value::Numeric(ref num, _) => num.format(self.format).fmt(out),
            Value::Color(ref rgba, ref name) => {
                if let Some(ref name) = *name {
                    name.fmt(out)
                } else {
                    rgba.format(self.format).fmt(out)
                }
            }
            Value::List(ref v, sep, brackets) => {
                let introspect = self.format.is_introspection();
                let t = v
                    .iter()
                    .filter(|v| !v.is_null() || introspect)
                    .map(|v| {
                        let needs_paren = match *v {
                            Value::List(ref v, inner, false) => {
                                ((brackets || introspect)
                                    && (sep == ListSeparator::Space
                                        || inner == ListSeparator::Comma))
                                    || (v.is_empty() && introspect)
                            }
                            _ => false,
                        };
                        if needs_paren {
                            format!("({})", v.format(self.format))
                        } else {
                            format!("{}", v.format(self.format))
                        }
                    })
                    .collect::<Vec<_>>();
                let t = if self.format.is_introspection()
                    && t.len() == 1
                    && sep == ListSeparator::Comma
                {
                    format!("{},", t[0])
                } else {
                    t.join(match sep {
                        ListSeparator::Comma => {
                            if self.format.is_compressed() {
                                ","
                            } else {
                                ", "
                            }
                        }
                        ListSeparator::Space => " ",
                    })
                };
                if brackets {
                    out.write_str("[")?;
                }
                write!(out, "{}", t)?;
                if brackets {
                    out.write_str("]")?;
                }
                Ok(())
            }
            Value::Call(ref name, ref arg) => {
                write!(out, "{}({})", name, arg)
            }
            Value::BinOp(ref a, _, Operator::Plus, _, ref b) => {
                // The plus operator is also a concat operator
                a.format(self.format).fmt(out)?;
                b.format(self.format).fmt(out)
            }
            Value::BinOp(ref a, ref s1, ref op, ref s2, ref b) => {
                a.format(self.format).fmt(out)?;
                if *s1 {
                    out.write_char(' ')?;
                }
                op.fmt(out)?;
                if *s2 {
                    out.write_char(' ')?;
                }
                b.format(self.format).fmt(out)
            }
            Value::UnaryOp(ref op, ref v) => {
                op.fmt(out)?;
                if *op == Operator::Not {
                    out.write_char(' ')?;
                }
                v.format(self.format).fmt(out)
            }
            Value::True => write!(out, "true"),
            Value::False => write!(out, "false"),
            Value::Null => {
                if self.format.is_introspection() {
                    out.write_str("null")
                } else {
                    Ok(())
                }
            }
            Value::Map(ref map) => write!(
                out,
                "({})",
                map.iter()
                    .map(|&(ref k, ref v)| format!(
                        "{}: {}",
                        k.format(self.format),
                        v.format(self.format)
                    ))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Value::UnicodeRange(ref s) => write!(out, "{}", s),
            Value::Paren(ref v) => {
                out.write_char('(')?;
                v.format(self.format).fmt(out)?;
                out.write_char(')')
            }
        }
    }
}

fn write_dq(out: &mut fmt::Formatter, s: &str) -> fmt::Result {
    out.write_char('"')?;
    for c in s.chars() {
        if c == '"' {
            out.write_char('\\')?;
        }
        out.write_char(c)?;
    }
    out.write_char('"')
}
fn write_sq(out: &mut fmt::Formatter, s: &str) -> fmt::Result {
    out.write_char('\'')?;
    for c in s.chars() {
        if c == '\'' {
            out.write_char('\\')?;
        }
        out.write_char(c)?;
    }
    out.write_char('\'')
}
