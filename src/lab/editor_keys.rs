use web_sys::{HtmlTextAreaElement, KeyboardEvent};

const INDENT: &str = "    ";

fn cursor_start(textarea: &HtmlTextAreaElement) -> u32 {
    textarea.selection_start().ok().flatten().unwrap_or(0)
}

fn cursor_end(textarea: &HtmlTextAreaElement) -> u32 {
    textarea
        .selection_end()
        .ok()
        .flatten()
        .unwrap_or_else(|| cursor_start(textarea))
}

pub fn handle_keydown(ev: &KeyboardEvent, textarea: &HtmlTextAreaElement) -> bool {
    match ev.key().as_str() {
        "Tab" => {
            handle_tab(ev.shift_key(), textarea);
            true
        }
        "Enter" if !ev.meta_key() && !ev.ctrl_key() => {
            handle_enter(textarea);
            true
        }
        "Backspace" => handle_backspace(textarea),
        _ => false,
    }
}

fn handle_tab(shift: bool, textarea: &HtmlTextAreaElement) {
    let value = textarea.value();
    let start = cursor_start(textarea);
    let end = cursor_end(textarea);

    if start != end {
        if indent_lines(textarea, &value, start, end, shift) {
            return;
        }
    }

    if shift {
        dedent_at_cursor(textarea, &value, start);
    } else {
        insert_at_cursor(textarea, &value, start, end, INDENT);
    }
}

fn handle_enter(textarea: &HtmlTextAreaElement) {
    let value = textarea.value();
    let pos = cursor_start(textarea);
    let indent = next_line_indent(&value, pos);
    insert_at_cursor(textarea, &value, pos, pos, &format!("\n{indent}"));
}

fn handle_backspace(textarea: &HtmlTextAreaElement) -> bool {
    let start = cursor_start(textarea);
    let end = cursor_end(textarea);
    if start != end {
        return false;
    }

    let value = textarea.value();
    let start = start as usize;
    let indent_len = INDENT.len();
    if start >= indent_len && value[(start - indent_len)..start] == *INDENT {
        let new_value = format!("{}{}", &value[..start - indent_len], &value[start..]);
        textarea.set_value(&new_value);
        set_selection(textarea, (start - indent_len) as u32);
        return true;
    }

    if start > 0 && value.as_bytes().get(start - 1) == Some(&b'\t') {
        let new_value = format!("{}{}", &value[..start - 1], &value[start..]);
        textarea.set_value(&new_value);
        set_selection(textarea, (start - 1) as u32);
        return true;
    }

    false
}

fn indent_lines(
    textarea: &HtmlTextAreaElement,
    value: &str,
    start: u32,
    end: u32,
    dedent: bool,
) -> bool {
    let start = start as usize;
    let end = end as usize;
    let line_start = value[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
    let line_end = value[end..]
        .find('\n')
        .map(|i| end + i)
        .unwrap_or(value.len());
    let block = &value[line_start..line_end];
    let lines: Vec<&str> = block.split('\n').collect();

    let new_lines: Vec<String> = if dedent {
        lines
            .iter()
            .map(|line| {
                if line.starts_with(INDENT) {
                    line[INDENT.len()..].to_string()
                } else if line.starts_with('\t') {
                    line[1..].to_string()
                } else {
                    (*line).to_string()
                }
            })
            .collect()
    } else {
        lines
            .iter()
            .map(|line| format!("{INDENT}{line}"))
            .collect()
    };

    let new_block = new_lines.join("\n");
    if new_block == block {
        return false;
    }

    let new_value = format!("{}{}{}", &value[..line_start], new_block, &value[line_end..]);
    textarea.set_value(&new_value);

    let delta = new_block.len() as i64 - block.len() as i64;
    let leading = start.saturating_sub(line_start) as i64;
    let new_start = if dedent {
        (line_start as i64 + leading - INDENT.len().min(leading as usize) as i64).max(line_start as i64)
    } else {
        line_start as i64 + leading + INDENT.len() as i64
    };
    let new_end = (end as i64 + delta).max(new_start);

    set_selection_range(textarea, new_start as u32, new_end as u32);
    true
}

fn dedent_at_cursor(textarea: &HtmlTextAreaElement, value: &str, start: u32) {
    let start = start as usize;
    let line_start = value[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
    let before = &value[line_start..start];

    if before.ends_with(INDENT) {
        let indent_len = INDENT.len();
        let new_value = format!("{}{}", &value[..start - indent_len], &value[start..]);
        textarea.set_value(&new_value);
        set_selection(textarea, (start - indent_len) as u32);
    } else if before.ends_with('\t') {
        let new_value = format!("{}{}", &value[..start - 1], &value[start..]);
        textarea.set_value(&new_value);
        set_selection(textarea, (start - 1) as u32);
    }
}

fn insert_at_cursor(
    textarea: &HtmlTextAreaElement,
    value: &str,
    start: u32,
    end: u32,
    insert: &str,
) {
    let start = start as usize;
    let end = end as usize;
    let new_value = format!("{}{}{}", &value[..start], insert, &value[end..]);
    textarea.set_value(&new_value);
    let pos = (start + insert.len()) as u32;
    set_selection(textarea, pos);
}

fn next_line_indent(value: &str, pos: u32) -> String {
    let pos = pos as usize;
    let line_start = value[..pos].rfind('\n').map(|i| i + 1).unwrap_or(0);
    let line = &value[line_start..pos];
    let base: String = line
        .chars()
        .take_while(|c| *c == ' ' || *c == '\t')
        .collect();
    let trimmed = line.trim_end();
    let head = trimmed.split(|c: char| c.is_whitespace() || c == '(').next().unwrap_or("");

    let needs_extra = trimmed.ends_with('(')
        || trimmed.ends_with('=')
        || matches!(
            head,
            "function" | "if" | "elseif" | "else" | "for" | "while" | "try" | "begin" | "do"
                | "struct" | "module" | "macro" | "quote"
        );

    format!("{base}{}", if needs_extra { INDENT } else { "" })
}

fn set_selection(textarea: &HtmlTextAreaElement, pos: u32) {
    set_selection_range(textarea, pos, pos);
}

fn set_selection_range(textarea: &HtmlTextAreaElement, start: u32, end: u32) {
    let _ = textarea.set_selection_start(Some(start));
    let _ = textarea.set_selection_end(Some(end));
}

pub fn focus_textarea(textarea: &HtmlTextAreaElement) {
    let _ = textarea.focus();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_line_indent_keeps_base_indent() {
        let value = "function foo()\n    x = 1\n    y";
        let pos = value.len() as u32;
        assert_eq!(next_line_indent(value, pos), "    ");
    }
}
