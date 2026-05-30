use lab_types::types::{BlockKind, LabModule};

use crate::markdown::markdown_to_rendered_html;

pub fn render_lab_briefs(mut module: LabModule) -> LabModule {
    for block in &mut module.blocks {
        if let BlockKind::Brief {
            body_md,
            brief_html,
            ..
        } = &mut block.kind
        {
            *brief_html = markdown_to_rendered_html(body_md);
            body_md.clear();
        }
    }
    module
}
