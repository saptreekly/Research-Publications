use crate::lab::types::{
    BlockExecution, BlockKind, LabBlock, LabModule, ProbeParam, VerifyArg, VerifyCase,
    VerifyExpectation,
};

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    EmptyDocument,
    UnclosedDirective { line: usize },
    UnknownDirective { name: String, line: usize },
    MissingBlockId { line: usize },
    InvalidProbeLine { line: String },
    InvalidVerifyLine { line: String },
}

pub fn parse_lab_module(module_id: &str, title: &str, source: &str) -> Result<LabModule, ParseError> {
    let mut blocks = Vec::new();
    let lines: Vec<&str> = source.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].trim();
        if let Some(rest) = line.strip_prefix(":::") {
            let rest = rest.trim();
            if rest.is_empty() {
                i += 1;
                continue;
            }
            if rest == "end" {
                i += 1;
                continue;
            }

            let (kind_name, attrs) = split_directive_header(rest);
            let start_line = i + 1;
            i += 1;

            let mut body_lines = Vec::new();
            while i < lines.len() {
                let body_line = lines[i];
                if body_line.trim() == ":::" || body_line.trim() == "::: end" {
                    break;
                }
                body_lines.push(body_line);
                i += 1;
            }

            if i >= lines.len() || !lines[i].trim().starts_with(":::") {
                return Err(ParseError::UnclosedDirective { line: start_line });
            }
            i += 1;

            let body = body_lines.join("\n");
            blocks.push(parse_directive(kind_name, attrs, body, start_line)?);
        } else {
            i += 1;
        }
    }

    if blocks.is_empty() {
        return Err(ParseError::EmptyDocument);
    }

    Ok(LabModule {
        id: module_id.to_string(),
        title: title.to_string(),
        blocks,
    })
}

fn split_directive_header(header: &str) -> (&str, &str) {
    match header.find(char::is_whitespace) {
        Some(idx) => (&header[..idx], header[idx..].trim()),
        None => (header, ""),
    }
}

fn parse_attrs(attrs: &str) -> Vec<(&str, &str)> {
    attrs
        .split_whitespace()
        .filter_map(|pair| pair.split_once('='))
        .collect()
}

fn require_id(attrs: &str, line: usize) -> Result<String, ParseError> {
    parse_attrs(attrs)
        .into_iter()
        .find(|(key, _)| *key == "id")
        .map(|(_, value)| value.to_string())
        .ok_or(ParseError::MissingBlockId { line })
}

fn optional_attr<'a>(attrs: &'a str, key: &str) -> Option<&'a str> {
    parse_attrs(attrs)
        .into_iter()
        .find(|(k, _)| *k == key)
        .map(|(_, value)| value)
}

fn parse_directive(
    kind_name: &str,
    attrs: &str,
    body: String,
    line: usize,
) -> Result<LabBlock, ParseError> {
    let execution = BlockExecution::default();

    match kind_name {
        "brief" => {
            let id = require_id(attrs, line)?;
            let title = optional_attr(attrs, "title").map(str::to_string);
            Ok(LabBlock {
                kind: BlockKind::Brief {
                    id,
                    title,
                    body_md: body.trim().to_string(),
                },
                execution,
            })
        }
        "probe" => {
            let id = require_id(attrs, line)?;
            let params = parse_probe_body(&body)?;
            Ok(LabBlock {
                kind: BlockKind::Probe { id, params },
                execution,
            })
        }
        "blueprint" => {
            let id = require_id(attrs, line)?;
            let language = optional_attr(attrs, "lang")
                .unwrap_or("julia")
                .to_string();
            let code = extract_fenced_code(&body, &language).unwrap_or_else(|| body.trim().to_string());
            Ok(LabBlock {
                kind: BlockKind::Blueprint {
                    id,
                    language,
                    code,
                },
                execution,
            })
        }
        "verify" => {
            let id = require_id(attrs, line)?;
            let cases = parse_verify_body(&body)?;
            Ok(LabBlock {
                kind: BlockKind::Verify { id, cases },
                execution,
            })
        }
        other => Err(ParseError::UnknownDirective {
            name: other.to_string(),
            line,
        }),
    }
}

fn parse_probe_body(body: &str) -> Result<Vec<ProbeParam>, ParseError> {
    let mut params = Vec::new();
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        params.push(parse_probe_line(line)?);
    }
    if params.is_empty() {
        return Err(ParseError::InvalidProbeLine {
            line: "Probe block requires at least one parameter.".to_string(),
        });
    }
    Ok(params)
}

fn parse_probe_line(line: &str) -> Result<ProbeParam, ParseError> {
    let (name, rest) = if let Some((name, value_part)) = line.split_once(':') {
        (name.trim(), value_part.trim())
    } else if let Some((name, value_part)) = line.split_once('=') {
        (name.trim(), value_part.trim())
    } else {
        return Err(ParseError::InvalidProbeLine {
            line: line.to_string(),
        });
    };

    if name.is_empty() {
        return Err(ParseError::InvalidProbeLine {
            line: line.to_string(),
        });
    }

    let mut value = None;
    let mut min = 0_i64;
    let mut max = 100_i64;

    for part in rest.split(',') {
        let part = part.trim();
        if let Some((key, val)) = part.split_once('=') {
            let key = key.trim();
            let val = val.trim().parse::<i64>().map_err(|_| ParseError::InvalidProbeLine {
                line: line.to_string(),
            })?;
            match key {
                "min" => min = val,
                "max" => max = val,
                _ => {}
            }
        } else if value.is_none() {
            value = Some(part.parse::<i64>().map_err(|_| ParseError::InvalidProbeLine {
                line: line.to_string(),
            })?);
        }
    }

    let value = value.ok_or_else(|| ParseError::InvalidProbeLine {
        line: line.to_string(),
    })?;

    if min > max {
        std::mem::swap(&mut min, &mut max);
    }

    Ok(ProbeParam {
        name: name.to_string(),
        value: value.clamp(min, max),
        min,
        max,
    })
}

fn parse_verify_body(body: &str) -> Result<Vec<VerifyCase>, ParseError> {
    let mut cases = Vec::new();
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if line.starts_with("- ") {
            cases.push(parse_verify_line(&line[2..])?);
        } else {
            cases.push(parse_verify_line(line)?);
        }
    }
    if cases.is_empty() {
        return Err(ParseError::InvalidVerifyLine {
            line: "Verify block requires at least one test case.".to_string(),
        });
    }
    Ok(cases)
}

fn parse_verify_line(line: &str) -> Result<VerifyCase, ParseError> {
    let (expression, expected) = if let Some(idx) = line.rfind(" throws") {
        (&line[..idx], VerifyExpectation::Error)
    } else if let Some((left, right)) = line.split_once(" == ") {
        let value = right.trim().parse::<i64>().map_err(|_| ParseError::InvalidVerifyLine {
            line: line.to_string(),
        })?;
        (left.trim(), VerifyExpectation::Value(value))
    } else {
        return Err(ParseError::InvalidVerifyLine {
            line: line.to_string(),
        });
    };

    let (function, args) = parse_call_expression(expression)?;
    Ok(VerifyCase {
        expression: line.to_string(),
        function,
        args,
        expected,
    })
}

fn parse_call_expression(expression: &str) -> Result<(String, Vec<VerifyArg>), ParseError> {
    let open = expression
        .find('(')
        .ok_or_else(|| ParseError::InvalidVerifyLine {
            line: expression.to_string(),
        })?;
    let close = expression
        .rfind(')')
        .ok_or_else(|| ParseError::InvalidVerifyLine {
            line: expression.to_string(),
        })?;

    let function = expression[..open].trim().to_string();
    let args_str = &expression[open + 1..close];
    let args = if args_str.trim().is_empty() {
        Vec::new()
    } else {
        args_str
            .split(',')
            .map(|arg| parse_verify_arg(arg.trim()))
            .collect::<Result<Vec<_>, _>>()?
    };

    Ok((function, args))
}

fn parse_verify_arg(arg: &str) -> Result<VerifyArg, ParseError> {
    if let Ok(value) = arg.parse::<i64>() {
        return Ok(VerifyArg::Literal(value));
    }
    if arg.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return Ok(VerifyArg::Probe(arg.to_string()));
    }
    Err(ParseError::InvalidVerifyLine {
        line: arg.to_string(),
    })
}

fn extract_fenced_code(body: &str, language: &str) -> Option<String> {
    let fence = format!("```{language}");
    let start = body.find(&fence)?;
    let after_fence = &body[start + fence.len()..];
    let end = after_fence.find("```")?;
    Some(after_fence[..end].trim_end().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_probe_and_verify_blocks() {
        let source = r#"
::: probe id=params
a: 3, min=1, max=20
n: 11, min=2, max=50
:::

::: verify id=exercise
- modInverse(3, 11) == 4
- modInverse(2, 4) throws
- modInverse(a, n) == 4
:::
"#;

        let module = parse_lab_module("mod_01", "Modular Foundations", source).unwrap();
        assert_eq!(module.blocks.len(), 2);
        assert_eq!(module.blocks[0].type_label(), "PROBE");
        assert_eq!(module.blocks[1].type_label(), "VERIFY");
    }
}
