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

/// Split markdown into `(heading, body)` pairs at `##` boundaries.
/// Content before the first `##` is returned with an empty heading.
pub fn split_h2_sections(source: &str) -> (String, Vec<(String, String)>) {
    let mut intro = String::new();
    let mut sections = Vec::new();
    let mut current_title = String::new();
    let mut current_body = String::new();
    let mut seen_h2 = false;

    for line in source.lines() {
        if let Some(title) = line.strip_prefix("## ") {
            if seen_h2 {
                sections.push((current_title.clone(), current_body.trim().to_string()));
            } else {
                intro = current_body.trim().to_string();
                seen_h2 = true;
            }
            current_title = title.trim().to_string();
            current_body.clear();
        } else {
            current_body.push_str(line);
            current_body.push('\n');
        }
    }

    if seen_h2 {
        sections.push((current_title, current_body.trim().to_string()));
    } else {
        intro = current_body.trim().to_string();
    }

    (intro, sections)
}

pub fn phase_id_from_heading(heading: &str) -> Option<&'static str> {
    if heading.contains("Phase 1") {
        Some("compromised")
    } else if heading.contains("Phase 2") {
        Some("loader")
    } else if heading.contains("Phase 3") {
        Some("clickfix")
    } else if heading.contains("Phase 4") {
        Some("payload")
    } else if heading.contains("Phase 5") {
        Some("rat")
    } else if heading.contains("Phase 6") {
        Some("netsupport")
    } else {
        None
    }
}

pub fn parse_detection_items(body: &str) -> Vec<(String, String)> {
    body.lines()
        .filter_map(|line| {
            let line = line.trim();
            let rest = line.strip_prefix("- **")?;
            let (category, text) = rest.split_once(":**")?;
            Some((category.trim().to_string(), text.trim().to_string()))
        })
        .collect()
}

pub fn parse_reference_items(body: &str) -> Vec<(String, Option<String>)> {
    body.lines()
        .filter_map(|line| {
            let line = line.trim();
            if !line.starts_with("- [") {
                return None;
            }
            let start = line.find('[')? + 1;
            let end = line.find(']')?;
            let label = line[start..end].to_string();
            let url = line
                .find("](")
                .map(|idx| line[idx + 2..].trim_end_matches(')').to_string());
            Some((label, url))
        })
        .chain(body.lines().filter_map(|line| {
            let line = line.trim();
            if line.starts_with("- [") || !line.starts_with("- ") {
                return None;
            }
            Some((line[2..].trim().to_string(), None))
        }))
        .collect()
}

#[derive(Clone, Debug)]
pub struct SigmaRulePreview {
    pub name: String,
    pub title: String,
    pub level: String,
    pub mitre: String,
    pub yaml: String,
}

pub fn parse_sigma_rules(source: &str) -> Vec<SigmaRulePreview> {
    let mut rules = Vec::new();
    let mut current_name = String::new();
    let mut in_yaml = false;
    let mut yaml_lines = Vec::new();

    for line in source.lines() {
        if let Some(name) = line.strip_prefix("### ") {
            if !current_name.is_empty() && !yaml_lines.is_empty() {
                rules.push(build_sigma_preview(&current_name, &yaml_lines));
                yaml_lines.clear();
            }
            current_name = name.trim().to_string();
            in_yaml = false;
            continue;
        }

        if line.trim() == "```yaml" {
            in_yaml = true;
            yaml_lines.clear();
            continue;
        }

        if in_yaml {
            if line.trim() == "```" {
                in_yaml = false;
                if !current_name.is_empty() {
                    rules.push(build_sigma_preview(&current_name, &yaml_lines));
                    yaml_lines.clear();
                }
            } else {
                yaml_lines.push(line.to_string());
            }
        }
    }

    rules
}

fn build_sigma_preview(name: &str, yaml_lines: &[String]) -> SigmaRulePreview {
    let yaml = yaml_lines.join("\n");
    let mut title = name.to_string();
    let mut level = "medium".to_string();
    let mut mitre = String::new();

    for line in yaml_lines {
        if let Some(value) = line.strip_prefix("title:") {
            title = value.trim().to_string();
        } else if let Some(value) = line.strip_prefix("level:") {
            level = value.trim().to_string();
        } else if line.contains("attack.t") {
            mitre = line
                .split('-')
                .last()
                .unwrap_or("")
                .trim()
                .to_string();
        }
    }

    SigmaRulePreview {
        name: name.to_string(),
        title,
        level,
        mitre,
        yaml,
    }
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
