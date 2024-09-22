use iced::widget::{button, checkbox, column, container, row, scrollable, text_input, text};
use iced::{Alignment, Element, Length};
use rfd::FileDialog;
use std::fs;
use std::path::PathBuf;
use arboard::Clipboard;

#[derive(Default)]
struct TreeGen {
    folder_path: String,
    root: Option<TreeNode>, // フォルダツリー全体を保持
    show_modal: bool,       // モーダルを表示するかどうか
}

#[derive(Debug, Clone)]
pub enum Message {
    FolderPathChanged(String),
    OpenFolderDialog,
    FolderSelected(Option<PathBuf>),
    FilterChanged(usize, bool),  // ノードのインデックスとチェック状態
    ShowModal(bool),
    ModalSelectionComplete,
    CopyToClipboard,
    AnalyzeTree,
}

#[derive(Debug, Clone)]
struct TreeNode {
    name: String,
    is_checked: bool,
    children: Vec<TreeNode>,
}

impl TreeNode {
    // 全ての子要素に対してチェックを伝播させる関数
    fn set_checked_recursive(&mut self, is_checked: bool) {
        self.is_checked = is_checked;
        for child in &mut self.children {
            child.set_checked_recursive(is_checked);
        }
    }

    // ノードから文字列を生成する（表示用）
    fn to_string_recursive(&self, depth: usize, is_last: bool, parent_is_last: &[bool]) -> String {
        let mut result = String::new();

        // 各階層に対するインデントを適切に追加
        for &is_parent_last in parent_is_last {
            result.push_str(if is_parent_last { "    " } else { "|   " });
        }

        result.push_str(if is_last { "|__ " } else { "|-- " });
        result.push_str(&self.name);
        result.push('\n');

        let len = self.children.len();
        for (i, child) in self.children.iter().enumerate() {
            let is_last_child = i == len - 1;
            // 再帰的に子ノードの文字列を生成（親の状態を引き継ぐ）
            result.push_str(&child.to_string_recursive(depth + 1, is_last_child, &[parent_is_last, &[is_last]].concat()));
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

            button("Analyze").on_press(Message::AnalyzeTree),

            // ツリー表示
            if let Some(root) = &self.root {
                let tree_text = root.to_string_recursive(0, true, &[]); // 修正箇所: is_lastを追加
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

        if self.show_modal {
            let modal_content = self.modal_view();
            return container(modal_content)
                .padding(20)
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(Alignment::Center)
                .into();
        }

        container(content)
            .padding(20)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::Center)
            .into()
    }

    fn modal_view(&self) -> Element<Message> {
        let mut nodes = Vec::new();
        if let Some(root) = &self.root {
            for (index, node) in root.children.iter().enumerate() {
                nodes.push(self.node_view(node, index));
            }
        }

        column![
            scrollable(column(nodes).spacing(10)),
            button("OK").on_press(Message::ModalSelectionComplete),
            button("Close").on_press(Message::ShowModal(false))
        ]
        .spacing(20)
        .align_x(Alignment::Center)
        .into()
    }

    fn node_view<'a>(&self, node: &'a TreeNode, depth: usize) -> Element<'a, Message> {
        column![
            row![
                checkbox("", node.is_checked).on_toggle(move |is_checked| Message::FilterChanged(depth, is_checked)),
                text(&node.name)
            ],
            column(node.children.iter().enumerate().map(|(_i, child)| self.node_view(child, depth + 1)).collect::<Vec<_>>())
        ]
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
            Message::AnalyzeTree => {
                let tree = self.generate_tree_structure();
                self.root = tree.ok();
                self.show_modal = true;
            }
            Message::FilterChanged(index, is_checked) => {
                if let Some(root) = self.root.as_mut() {
                    TreeGen::update_tree_checked(root, index, is_checked);
                }
            }
            Message::ShowModal(show) => {
                self.show_modal = show;
            }
            Message::ModalSelectionComplete => {
                self.show_modal = false;
            }
            Message::CopyToClipboard => {
                if let Some(root) = &self.root {
                    let mut clipboard = Clipboard::new().unwrap();
                    clipboard.set_text(root.to_string_recursive(0, true, &[])).unwrap();
                }
            }
            // 未処理だったFolderSelectedを追加
            Message::FolderSelected(_) => {
                // フォルダ選択時の処理、特に何もしないなら空のブロックを追加
            }
        }
    }

    fn update_tree_checked(node: &mut TreeNode, _depth: usize, is_checked: bool) {
        node.set_checked_recursive(is_checked);
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
        is_checked: true, // デフォルトでは全て選択された状態
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
