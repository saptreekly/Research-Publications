use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use render_core::lab::modules::ALL_MODULES;
use render_core::lab::parse::parse_lab_module;
use render_core::lab::render::render_lab_briefs;
use render_core::markdown::{
    build_malware_report_cache, markdown_to_rendered_html, rendered_html_path,
};

fn main() {
    let root = PathBuf::from("research-docs");
    if !root.is_dir() {
        eprintln!("research-docs/ not found; skipping pre-render");
        return;
    }

    let mut rendered = 0u32;
    let mut skipped = 0u32;

    rendered += render_markdown_tree(&root, &mut skipped);
    rendered += render_malware_caches(&root.join("malware-traffic"), &mut skipped);
    rendered += render_lab_caches(&mut skipped);

    eprintln!("prebuild: {rendered} updated, {skipped} skipped (static/rendered/)");
}

fn is_stale(output: &Path, sources: &[&Path]) -> bool {
    let Ok(out_meta) = fs::metadata(output) else {
        return true;
    };
    let Ok(out_modified) = out_meta.modified() else {
        return true;
    };

    for source in sources {
        let Ok(src_meta) = fs::metadata(source) else {
            return true;
        };
        let Ok(src_modified) = src_meta.modified() else {
            return true;
        };
        if src_modified > out_modified {
            return true;
        }
    }

    false
}

fn write_if_stale(output: &Path, sources: &[&Path], content: &str) -> bool {
    if !is_stale(output, sources) {
        return false;
    }
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent).expect("create output dir");
    }
    let temp = output.with_extension("tmp");
    {
        let mut file = fs::File::create(&temp).expect("create temp output");
        file.write_all(content.as_bytes()).expect("write temp output");
    }
    fs::rename(temp, output).expect("rename output");
    true
}

fn render_lab_caches(skipped: &mut u32) -> u32 {
    let mut updated = 0;
    for module in ALL_MODULES {
        let source_path = Path::new(module.lab_src);
        let out = PathBuf::from(format!("static/rendered/lab/{}.json", module.id));
        if !is_stale(&out, &[source_path]) {
            *skipped += 1;
            continue;
        }

        let source = fs::read_to_string(source_path).unwrap_or_else(|_| panic!("read {}", module.lab_src));
        let parsed = parse_lab_module(module.id, module.title, &source)
            .unwrap_or_else(|err| panic!("parse {}: {err:?}", module.lab_src));
        let rendered = render_lab_briefs(parsed);
        let json = serde_json::to_string_pretty(&rendered).expect("serialize lab cache");
        if write_if_stale(&out, &[source_path], &json) {
            updated += 1;
        } else {
            *skipped += 1;
        }
    }
    updated
}

fn render_markdown_tree(root: &Path, skipped: &mut u32) -> u32 {
    let mut updated = 0;
    for entry in walkdir::WalkDir::new(root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.path().extension().is_some_and(|ext| ext == "md"))
    {
        let path = entry.path();
        let relative = path
            .strip_prefix("research-docs")
            .expect("markdown under research-docs")
            .to_string_lossy()
            .replace('\\', "/");
        let md_path = format!("research-docs/{relative}");
        let out = PathBuf::from(rendered_html_path(&md_path));

        if !is_stale(&out, &[path]) {
            *skipped += 1;
            continue;
        }

        let source = fs::read_to_string(path).expect("read markdown source");
        let html = markdown_to_rendered_html(&source);
        if write_if_stale(&out, &[path], &html) {
            updated += 1;
        } else {
            *skipped += 1;
        }
    }
    updated
}

fn render_malware_caches(dir: &Path, skipped: &mut u32) -> u32 {
    if !dir.is_dir() {
        return 0;
    }

    let mut updated = 0;
    let mut analysis_by_stem: std::collections::HashMap<String, (PathBuf, String)> =
        std::collections::HashMap::new();
    let mut sigma_by_stem: std::collections::HashMap<String, (PathBuf, String)> =
        std::collections::HashMap::new();

    for entry in fs::read_dir(dir).expect("read malware-traffic dir") {
        let path = entry.expect("malware-traffic entry").path();
        if path.extension().is_none_or(|ext| ext != "md") {
            continue;
        }
        let file_name = path.file_name().unwrap().to_string_lossy();
        let source = fs::read_to_string(&path).expect("read malware markdown");
        if file_name.ends_with("-sigma.md") {
            let stem = file_name.trim_end_matches("-sigma.md").to_string();
            sigma_by_stem.insert(stem, (path, source));
        } else if file_name.ends_with("-ioc.md") {
            continue;
        } else {
            let stem = file_name.trim_end_matches(".md").to_string();
            analysis_by_stem.insert(stem, (path, source));
        }
    }

    for (stem, (analysis_path, analysis)) in analysis_by_stem {
        let sigma = sigma_by_stem.get(&stem);
        let out = PathBuf::from(format!("static/rendered/malware-traffic/{stem}.report.json"));
        let mut sources: Vec<&Path> = vec![&analysis_path];
        if let Some((sigma_path, _)) = sigma {
            sources.push(sigma_path.as_path());
        }

        if !is_stale(&out, &sources) {
            *skipped += 1;
            continue;
        }

        let sigma_text = sigma.map(|(_, text)| text.as_str());
        let cache = build_malware_report_cache(&analysis, sigma_text);
        let json = serde_json::to_string_pretty(&cache).expect("serialize malware report cache");
        if write_if_stale(&out, &sources, &json) {
            updated += 1;
        } else {
            *skipped += 1;
        }
    }
    updated
}
