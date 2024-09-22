use std::fs;
use std::path::PathBuf;

// フォルダツリーを再帰的に生成する関数
pub fn generate_tree_structure(root: &str) -> Result<String, std::io::Error> {
    let mut result = String::new();
    let root_path = PathBuf::from(root);
    generate_tree_recursive(&root_path, 0, &mut result, &mut Vec::new())?;
    Ok(result)
}

// 再帰的にフォルダツリーを構築する関数
fn generate_tree_recursive(
    path: &PathBuf,
    depth: usize,
    result: &mut String,
    is_last_stack: &mut Vec<bool>
) -> std::io::Result<()> {
    if path.is_dir() {
        let entries: Vec<_> = fs::read_dir(path)?.collect();
        let entries_len = entries.len();
        for (i, entry) in entries.into_iter().enumerate() {
            let entry = entry?;
            let entry_path = entry.path();
            let entry_name = entry.file_name().into_string().unwrap_or_default();

            // インデントを深さに応じて追加
            for &is_last in is_last_stack.iter() {
                if is_last {
                    result.push_str("    ");
                } else {
                    result.push_str("|   ");
                }
            }

            if i == entries_len - 1 {
                result.push_str("|__ ");
                is_last_stack.push(true);
            } else {
                result.push_str("|-- ");
                is_last_stack.push(false);
            }

            result.push_str(&format!("{}\n", entry_name));

            if entry_path.is_dir() {
                generate_tree_recursive(&entry_path, depth + 1, result, is_last_stack)?;
            }

            is_last_stack.pop();
        }
    }
    Ok(())
}
