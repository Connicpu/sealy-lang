use std::borrow::Cow;
use std::char::from_u32;

pub fn parse_literal(lit: &str) -> Result<Cow<str>, &'static str> {
    let lit = &lit[1..lit.len() - 1];
    if !lit.contains('\\') {
        return Ok(Cow::Borrowed(lit));
    }

    let mut buffer = String::with_capacity(lit.len());
    let mut iter = lit.chars();

    'iter: while let Some(c) = iter.next() {
        if c == '\\' {
            let c = match iter.next() {
                Some(c) => c,
                None => return Err("Unexpected end of escape sequence"),
            };

            let c = match c {
                '0' => '\0',
                'n' => '\n',
                'r' => '\r',
                't' => '\t',
                '\\' => '\\',
                '\'' => '\'',
                '"' => '"',
                'u' => {
                    let mut value = 0;
                    if iter.next() != Some('{') {
                        return Err("Unexpected end of escape sequence");
                    }

                    let res;
                    loop {
                        let c = match iter.next() {
                            Some(c) => c,
                            None => return Err("Unexpected end of escape sequence"),
                        };

                        if c == '}' {
                            res = match from_u32(value) {
                                Some(c) => c,
                                None => return Err("Unicode escape was not a valid codepoint"),
                            };
                            break;
                        }
                        if let Some(v) = c.to_digit(16) {
                            value <<= 4;
                            value |= v;
                        } else {
                            return Err("Unicode escape contains non-hex digit");
                        }
                    }
                    res
                }
                _ => return Err("Unknown escape character"),
            };

            buffer.push(c);
        } else {
            buffer.push(c);
        }
    }

    buffer.shrink_to_fit();
    Ok(Cow::Owned(buffer))
}
