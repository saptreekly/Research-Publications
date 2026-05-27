use comrak::{markdown_to_html, Options};

const PLACEHOLDER_PREFIX: &str = "XMATHPH";

/// Extract `$...$` and `$$...$$` so comrak does not mangle underscores inside math.
pub fn extract_math(source: &str) -> (String, Vec<String>) {
    let mut protected = String::new();
    let mut blocks = Vec::new();
    let chars: Vec<char> = source.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if chars[i] == '$' {
            let display = i + 1 < chars.len() && chars[i + 1] == '$';
            let start = if display { i + 2 } else { i + 1 };
            let mut j = start;
            let mut found = false;

            while j < chars.len() {
                if chars[j] == '$' {
                    if display {
                        if j + 1 < chars.len() && chars[j + 1] == '$' {
                            let tex: String = chars[start..j].iter().collect();
                            blocks.push(if display {
                                format!("$${tex}$$")
                            } else {
                                format!("${tex}$")
                            });
                            let idx = blocks.len() - 1;
                            protected.push_str(&format!("{PLACEHOLDER_PREFIX}{idx}END"));
                            i = j + 2;
                            found = true;
                            break;
                        }
                    } else {
                        let tex: String = chars[start..j].iter().collect();
                        blocks.push(format!("${tex}$"));
                        let idx = blocks.len() - 1;
                        protected.push_str(&format!("{PLACEHOLDER_PREFIX}{idx}END"));
                        i = j + 1;
                        found = true;
                        break;
                    }
                }
                j += 1;
            }

            if !found {
                protected.push(chars[i]);
                i += 1;
            }
        } else {
            protected.push(chars[i]);
            i += 1;
        }
    }

    (protected, blocks)
}

pub fn restore_math(html: &str, blocks: &[String]) -> String {
    let mut restored = html.to_string();
    for (idx, block) in blocks.iter().enumerate() {
        let placeholder = format!("{PLACEHOLDER_PREFIX}{idx}END");
        restored = restored.replace(&placeholder, block);
    }
    restored
}

pub fn markdown_to_rendered_html(source: &str) -> String {
    let (protected, blocks) = extract_math(source);
    let mut options = Options::default();
    options.extension.table = true;
    let html = markdown_to_html(&protected, &options);
    restore_math(&html, &blocks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn protects_inline_math_from_comrak_emphasis() {
        let md = "Inverse $a^{-1}$ modulo $n$.";
        let html = markdown_to_rendered_html(md);
        assert!(html.contains("$a^{-1}$"));
        assert!(!html.contains("<em>"));
    }

    #[test]
    fn preserves_display_math_delimiters() {
        let md = "Formula:\n\n$$ax \\equiv 1 \\pmod n$$\n\nDone.";
        let html = markdown_to_rendered_html(md);
        assert!(html.contains("$$ax \\equiv 1 \\pmod n$$"));
    }

    #[test]
    fn round_trips_multiple_expressions() {
        let md = "$\\gcd(a,m) = 1$ and $a \\cdot x \\equiv 1 \\pmod{m}$";
        let html = markdown_to_rendered_html(md);
        assert!(html.contains("$\\gcd(a,m) = 1$"));
        assert!(html.contains("$a \\cdot x \\equiv 1 \\pmod{m}$"));
    }
}
