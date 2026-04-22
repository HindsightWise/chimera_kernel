use std::fs;
use std::path::{Path, PathBuf};

fn get_all_md_files(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if !dir.exists() { return files; }
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                files.extend(get_all_md_files(&path));
            } else if path.extension().and_then(|s| s.to_str()) == Some("md") {
                files.push(path);
            }
        }
    }
    files.sort();
    files
}

fn merge_files(target: &str, source_files: &[PathBuf]) {
    let mut out_content = format!("# {}\n\n> [!NOTE]\n> This is a Topologically Flattened AI-First Macro-Document.\n\n", target.replace(".md", "").replace("_", " "));
    
    for file in source_files {
        let file_str = file.to_string_lossy();
        if file_str.contains("CURRENT_CONTEXT") || file_str.contains("GLOSSOPETRAE") || file_str.contains("ACTIVE_STATE") {
            continue;
        }
        
        if let Ok(content) = fs::read_to_string(file) {
            out_content.push_str(&format!("\n\n<!-- ========================================== -->\n"));
            out_content.push_str(&format!("<!-- SOURCE FILE: {} -->\n", file_str));
            out_content.push_str(&format!("<!-- ========================================== -->\n\n"));
            out_content.push_str(&content);
            out_content.push_str("\n\n");
        }
    }
    
    let _ = fs::write(target, out_content);
}

fn main() {
    let base = Path::new("/Users/zerbytheboss/Monad");
    std::env::set_current_dir(base).unwrap();
    
    let mut arch_files = vec![
        PathBuf::from("README.md"),
        PathBuf::from(".monad_axioms.md"),
        PathBuf::from("MONAD_WBS.md"),
    ];
    arch_files.extend(get_all_md_files(Path::new("KNOWLEDGE/SYSTEM")));
    arch_files.extend(get_all_md_files(Path::new("KNOWLEDGE/EXTERNAL")));
    
    // Deduplicate
    let mut unique_arch = Vec::new();
    for f in arch_files {
        if !unique_arch.contains(&f) {
            unique_arch.push(f);
        }
    }
    merge_files("MONAD_ARCHITECTURE.md", &unique_arch);
    
    let ops_files = get_all_md_files(Path::new("KNOWLEDGE/OPERATIONAL"));
    merge_files("MONAD_OPERATIONS.md", &ops_files);
    
    let res_files = get_all_md_files(Path::new("KNOWLEDGE/RESEARCH"));
    merge_files("MONAD_RESEARCH_ARCHIVE.md", &res_files);
    
    println!("Flattening complete!");
}
