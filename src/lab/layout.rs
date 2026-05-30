use lab_types::types::{BlockKind, LabBlock, ProbeParam, VerifyCase};

pub struct ParsedLab {
    pub briefs: Vec<(Option<String>, String)>,
    pub probe: Option<Vec<ProbeParam>>,
    pub verify_cases: Vec<VerifyCase>,
    pub starter_code: String,
}

pub fn parse_lab_blocks(blocks: &[LabBlock]) -> ParsedLab {
    let mut briefs = Vec::new();
    let mut probe = None;
    let mut verify_cases = Vec::new();
    let mut starter_code = String::new();
    let mut blueprint_code = String::new();

    for block in blocks {
        match &block.kind {
            BlockKind::Brief {
                title,
                brief_html,
                ..
            } => briefs.push((title.clone(), brief_html.clone())),
            BlockKind::Probe { params, .. } => probe = Some(params.clone()),
            BlockKind::Verify { cases, .. } => verify_cases = cases.clone(),
            BlockKind::Starter { code, .. } if starter_code.is_empty() => {
                starter_code = code.clone();
            }
            BlockKind::Blueprint { code, .. } if blueprint_code.is_empty() => {
                blueprint_code = code.clone();
            }
            _ => {}
        }
    }

    if starter_code.is_empty() {
        starter_code = fallback_starter(&blueprint_code);
    }

    ParsedLab {
        briefs,
        probe,
        verify_cases,
        starter_code,
    }
}

fn fallback_starter(blueprint: &str) -> String {
    if blueprint.trim().is_empty() {
        return "# Write your Julia solution here\n".to_string();
    }

    blueprint
        .lines()
        .filter(|line| {
            let trimmed = line.trim();
            !trimmed.starts_with("println(") && !trimmed.starts_with("using ")
        })
        .collect::<Vec<_>>()
        .join("\n")
}
