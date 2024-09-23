use iced::widget::{button, column, container, row, scrollable, text_input, text};
use iced::{Alignment, Element, Length};
use rfd::FileDialog;
use std::fs;
use std::path::PathBuf;
use arboard::Clipboard;

#[derive(Default)]
struct TreeGen {
    folder_path: String,
    root: Option<TreeNode>, // フォルダツリー全体を保持
}

#[derive(Debug, Clone)]
pub enum Message {
    FolderPathChanged(String),
    OpenFolderDialog,
    FolderSelected(Option<PathBuf>),
    CopyToClipboard,
    GenerateTree,
}

#[derive(Debug, Clone)]
struct TreeNode {
    name: String,
    children: Vec<TreeNode>,
}

impl TreeNode {
    // ルートディレクトリにはインデントや接頭辞をつけない
    fn to_string_recursive(&self, depth: usize, is_last: bool, parent_is_last: &[bool], is_root: bool) -> String {
        let mut result = String::new();

        if is_root {
            // ルートの場合はそのまま名前を表示
            result.push_str(&self.name);
            result.push('\n');
        } else {
            // ルート以外のノードにはインデントと接頭辞をつける
            for &is_parent_last in parent_is_last {
                result.push_str(if is_parent_last { "    " } else { "|   " });
            }

            result.push_str(if is_last { "|__ " } else { "|-- " });
            result.push_str(&self.name);
            result.push('\n');
        }

        let len = self.children.len();
        for (i, child) in self.children.iter().enumerate() {
            let is_last_child = i == len - 1;
            result.push_str(&child.to_string_recursive(depth + 1, is_last_child, &[parent_is_last, &[is_last]].concat(), false));
        }

        result
    }
}

impl TreeGen {
    pub fn view(&self) -> Element<Message> {
        let content = column![
            row![
                text_input("Enter folder path...", &self.folder_path)
                    .padding(10)
                    .width(Length::Fill)
                    .on_input(Message::FolderPathChanged),
                button("Browse").on_press(Message::OpenFolderDialog)
            ]
            .spacing(10),

            button("Generate Tree").on_press(Message::GenerateTree), // Generate Treeボタンを追加

            // ツリー表示
            if let Some(root) = &self.root {
                let tree_text = root.to_string_recursive(0, true, &[], true); // ルートディレクトリにはインデントをつけない
                scrollable(text(tree_text))
                    .width(Length::Fill)
                    .height(Length::Fill)
            } else {
                scrollable(column![]).width(Length::Fill).height(Length::Fill)
            },

            button("Copy to Clipboard").on_press(Message::CopyToClipboard)
        ]
        .spacing(20)
        .align_x(Alignment::Center);

        container(content)
            .padding(20)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::Center)
            .into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::FolderPathChanged(new_path) => {
                self.folder_path = new_path;
            }
            Message::OpenFolderDialog => {
                let selected_folder = FileDialog::new()
                    .set_title("Browse")
                    .pick_folder();

                if let Some(path) = selected_folder {
                    self.folder_path = path.display().to_string();
                }
            }
            Message::GenerateTree => {
                let tree = self.generate_tree_structure();
                self.root = tree.ok();
            }
            Message::CopyToClipboard => {
                if let Some(root) = &self.root {
                    let mut clipboard = Clipboard::new().unwrap();
                    clipboard.set_text(root.to_string_recursive(0, true, &[], true)).unwrap();
                }
            }
            Message::FolderSelected(_) => {
                // フォルダ選択時の処理、特に何もしないなら空のブロックを追加
            }
        }
    }

    fn generate_tree_structure(&self) -> Result<TreeNode, std::io::Error> {
        let root_path = PathBuf::from(&self.folder_path);
        let root_node = generate_tree_recursive(&root_path)?;
        Ok(root_node)
    }
}

// 再帰的にフォルダツリーを構築する関数
fn generate_tree_recursive(path: &PathBuf) -> std::io::Result<TreeNode> {
    let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
    let mut node = TreeNode {
        name,
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

fn main() -> iced::Result {
    iced::run("TreeGen", TreeGen::update, TreeGen::view)
}
