use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct TreeNode {
    pub id: Uuid, // 各ノードに一意のIDを付与
    pub name: String,
    pub is_checked: bool,
    pub children: Vec<TreeNode>,
}

impl TreeNode {
    // 子ノードのチェック状態を再帰的に変更する関数
    pub fn set_checked_recursive(&mut self, is_checked: bool) {
        self.is_checked = is_checked;
        for child in &mut self.children {
            child.set_checked_recursive(is_checked);
        }
    }

    // ノードのUUIDに基づいてチェック状態を変更する関数
    pub fn update_check_by_id(&mut self, target_id: &Uuid, is_checked: bool) {
        if &self.id == target_id {
            self.set_checked_recursive(is_checked);
        } else {
            for child in &mut self.children {
                child.update_check_by_id(target_id, is_checked);
            }
        }
    }

    // チェックされたノードのみを再帰的にツリーとして表示する関数
    pub fn to_string_recursive(&self, depth: usize, is_last: bool, parent_is_last: &[bool]) -> String {
        let mut result = String::new();

        // 各階層に対するインデントを適切に追加
        if depth > 0 {
            for &is_parent_last in parent_is_last.iter().take(depth - 1) {
                result.push_str(if is_parent_last { "    " } else { "|   " });
            }
            result.push_str(if is_last { "|__ " } else { "|-- " });
        }

        result.push_str(&self.name);
        result.push('\n');

        let len = self.children.len();
        for (i, child) in self.children.iter().enumerate() {
            let is_last_child = i == len - 1;
            result.push_str(&child.to_string_recursive(
                depth + 1,
                is_last_child,
                &[parent_is_last, &[is_last_child]].concat(),
            ));
        }

        result
    }

    // フィルタを適用する関数
    pub fn apply_filter(&mut self) {
        self.children.retain(|child| child.is_checked); // チェックされている子だけ残す
        for child in &mut self.children {
            child.apply_filter(); // 再帰的にフィルタを適用
        }
    }
}

/// 再帰的にフォルダツリーを構築する関数
pub fn generate_tree_recursive(path: &PathBuf) -> std::io::Result<TreeNode> {
    let name = path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
    let mut node = TreeNode {
        id: Uuid::new_v4(), // 一意のUUIDを生成
        name,
        is_checked: true, // デフォルトでは全てチェックされた状態
        children: Vec::new(),
    };

    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let child_node = generate_tree_recursive(&entry.path())?;
            node.children.push(child_node);
        }
    }

    Ok(node)
}
