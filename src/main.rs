use iced::widget::{button, column, container, scrollable, text, text_input};
use iced::{Alignment, Element, Length};
use std::fs;
use std::path::PathBuf;

#[derive(Default)]
struct TreeGen {
    folder_path: String,      // ユーザーが入力したフォルダパス
    tree_structure: String,   // 生成されたフォルダツリー
}

#[derive(Debug, Clone)]
pub enum Message {
    FolderPathChanged(String),  // フォルダパスの変更
    GenerateTree,               // ツリー生成ボタンが押された
}

impl TreeGen {
    // ビューの定義
    pub fn view(&self) -> Element<Message> {
        let scrollable_tree = scrollable(
            text(&self.tree_structure)
                .size(16)
        )
        .width(Length::Fill)
        .height(Length::Fill);

        let content = column![
            // フォルダパス入力フィールド
            text_input("Enter folder path...", &self.folder_path)
                .padding(10)
                .width(Length::Fill)
                .on_input(Message::FolderPathChanged),

            // ツリー構造を生成するボタン
            button("Generate Tree").on_press(Message::GenerateTree),

            // スクロール可能なツリー表示領域
            scrollable_tree
        ]
        .spacing(20)
        .align_x(Alignment::Center);

        // 全体を中央に配置
        container(content)
            .padding(20)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::Center)
            .into()
    }

    // 更新処理の定義
    pub fn update(&mut self, message: Message) {
        match message {
            Message::FolderPathChanged(new_path) => {
                self.folder_path = new_path;
            }
            Message::GenerateTree => {
                // フォルダツリーを生成
                if !self.folder_path.is_empty() {
                    let tree = generate_tree_structure(&self.folder_path);
                    self.tree_structure = tree.unwrap_or_else(|err| err.to_string());
                }
            }
        }
    }
}

// フォルダツリーを再帰的に生成する関数
fn generate_tree_structure(root: &str) -> Result<String, std::io::Error> {
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

// メイン関数
fn main() -> iced::Result {
    iced::run("TreeGen", TreeGen::update, TreeGen::view)
}
