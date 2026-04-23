use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

const CATALOG_DIRS: &[&str] = &[
    "scenarios",
    "workflows",
    "roles",
    "artifacts",
    "checks",
    "skills",
];

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("manifest dir"));
    let workspace_root = manifest_dir
        .parent()
        .and_then(Path::parent)
        .expect("workspace root");

    for dir in CATALOG_DIRS {
        println!(
            "cargo:rerun-if-changed={}",
            workspace_root.join(dir).display()
        );
    }

    let mut entries = Vec::new();
    for dir in CATALOG_DIRS {
        let dir_path = workspace_root.join(dir);
        for entry in WalkDir::new(&dir_path).into_iter().filter_map(Result::ok) {
            if !entry.file_type().is_file() {
                continue;
            }

            let path = entry.path();
            if !should_embed(dir, path) {
                continue;
            }
            let relative = path
                .strip_prefix(workspace_root)
                .expect("catalog files under workspace root");
            let relative = relative.to_string_lossy().replace('\\', "/");
            let contents = fs::read_to_string(path).expect("catalog file readable");
            entries.push((relative, contents));
        }
    }

    entries.sort_by(|left, right| left.0.cmp(&right.0));

    let generated = render_entries(&entries);
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("out dir"));
    fs::write(out_dir.join("embedded_catalog.rs"), generated).expect("embedded catalog generated");
}

fn should_embed(top_dir: &str, path: &Path) -> bool {
    match top_dir {
        "skills" => path.file_name().and_then(|name| name.to_str()) == Some("SKILL.md"),
        _ => path.extension().and_then(|extension| extension.to_str()) == Some("md"),
    }
}

fn render_entries(entries: &[(String, String)]) -> String {
    let mut output =
        String::from("pub static EMBEDDED_CATALOG_FILES: &[EmbeddedCatalogFile] = &[\n");
    for (path, contents) in entries {
        output.push_str("    EmbeddedCatalogFile {\n");
        output.push_str(&format!("        path: {:?},\n", path));
        output.push_str(&format!("        contents: {:?},\n", contents));
        output.push_str("    },\n");
    }
    output.push_str("];\n");
    output
}
