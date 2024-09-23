use iced::widget::{button, checkbox, column, container, row, scrollable, text_input, text};
use iced::{Alignment, Element, Length};
use rfd::FileDialog;
use std::fs;
use std::path::PathBuf;
use arboard::Clipboard;
use uuid::Uuid;

#[derive(Default)]
struct TreeGen {
    folder_path: String,
    root: Option<TreeNode>, // フォルダツリー全体を保持
    show_filter_modal: bool, // フィルターモーダルを表示するかどうか
    filter_root: Option<TreeNode>, // フィルタリング用のツリー
}

#[derive(Debug, Clone)]
pub enum Message {
    FolderPathChanged(String),
    OpenFolderDialog,
    FolderSelected(Option<PathBuf>),
    ApplyFilter, // フィルタリングを適用
    ToggleNodeCheck(Uuid, bool), // ノードのUUIDとチェック状態
    ShowFilterModal(bool), // フィルタモーダルの表示/非表示を切り替える
    CopyToClipboard,
}

#[derive(Debug, Clone)]
struct TreeNode {
    id: Uuid, // 各ノードに一意のIDを付与
    name: String,
    is_checked: bool,
    children: Vec<TreeNode>,
}

impl TreeNode {
    // 子ノードのチェック状態を再帰的に変更する関数
    fn set_checked_recursive(&mut self, is_checked: bool) {
        self.is_checked = is_checked;
        for child in &mut self.children {
            child.set_checked_recursive(is_checked);
        }
    }

    // ノードのUUIDに基づいてチェック状態を変更する関数
    fn update_check_by_id(&mut self, target_id: &Uuid, is_checked: bool) {
        if &self.id == target_id {
            self.set_checked_recursive(is_checked);
        } else {
            for child in &mut self.children {
                child.update_check_by_id(target_id, is_checked);
            }
        }
    }

    // チェックされたノードのみを再帰的にツリーとして表示する関数
    fn to_string_recursive(&self, depth: usize, is_last: bool, parent_is_last: &[bool]) -> String {
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
    fn apply_filter(&mut self) {
        self.children.retain(|child| child.is_checked); // チェックされている子だけ残す
        for child in &mut self.children {
            child.apply_filter(); // 再帰的にフィルタを適用
        }
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
            button("Filter").on_press(Message::ShowFilterModal(true)), // フィルタモーダルを表示
            // パスが入力されたらツリー表示
            if let Some(root) = &self.root {
                let tree_text = root.to_string_recursive(0, true, &[]);
                scrollable(text(tree_text))
                    .width(Length::Fill)
                    .height(Length::Fill)
            } else {
                scrollable(column![]).width(Length::Fill).height(Length::Fill)
            },
            row![
                button("Copy to Clipboard").on_press(Message::CopyToClipboard)
            ]
        ]
        .spacing(20)
        .align_x(Alignment::Center);

        let main_content = container(content)
            .padding(20)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::Center)
            .into();

        // フィルタ画面のモーダル表示
        if self.show_filter_modal {
            let modal_content = self.filter_modal_view();
            return container(modal_content)
                .padding(20)
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(Alignment::Center)
                .into();
        }

        main_content
    }

    // フィルタリング用のモーダルを描画
    fn filter_modal_view(&self) -> Element<Message> {
        let filter_content = column![
            text("Filter your Tree").size(30),
            // フィルタリングツリーの表示
            if let Some(filter_root) = &self.filter_root {
                scrollable(self.render_filter_tree_view(filter_root))
                    .width(Length::Fill)
                    .height(Length::Fill)
            } else {
                scrollable(column![]).width(Length::Fill).height(Length::Fill)
            },
            row![
                button("OK").on_press(Message::ApplyFilter),
                button("Cancel").on_press(Message::ShowFilterModal(false)) // キャンセルボタンでモーダルを閉じる
            ]
            .spacing(20)
        ]
        .spacing(20)
        .align_x(Alignment::Center);

        container(filter_content)
            .padding(20)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::Center)
            .into()
    }

    // フィルタリング用のチェック付きツリーを描画
    fn render_filter_tree_view<'a>(&self, node: &'a TreeNode) -> Element<'a, Message> {
        let children_elements: Vec<Element<Message>> = node
            .children
            .iter()
            .map(|child| self.render_filter_tree_view(child))
            .collect();

        column![
            row![
                checkbox("", node.is_checked).on_toggle({
                    let node_id = node.id;
                    move |is_checked| Message::ToggleNodeCheck(node_id, is_checked)
                }),
                text(&node.name)
            ],
            column(children_elements).padding(10)
        ]
        .into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::FolderPathChanged(new_path) => {
                self.folder_path = new_path;
            }
            Message::OpenFolderDialog => {
                let selected_folder = FileDialog::new().set_title("Browse").pick_folder();

                if let Some(path) = selected_folder {
                    self.folder_path = path.display().to_string();
                    let tree = self.generate_tree_structure();
                    self.root = tree.ok();
                    self.filter_root = self.root.clone(); // フィルタリング用にコピー
                }
            }
            Message::ToggleNodeCheck(node_id, is_checked) => {
                if let Some(filter_root) = &mut self.filter_root {
                    // ノードのUUIDに基づいてチェック状態を変更
                    filter_root.update_check_by_id(&node_id, is_checked);
                }
            }
            Message::ApplyFilter => {
                // フィルタ適用処理
                if let Some(filter_root) = &mut self.filter_root {
                    filter_root.apply_filter(); // チェックされた項目のみ保持
                    self.root = Some(filter_root.clone()); // フィルタ結果を反映
                }
                self.show_filter_modal = false; // フィルタリングが終わったらモーダルを閉じる
            }
            Message::ShowFilterModal(show) => {
                self.show_filter_modal = show;
            }
            Message::CopyToClipboard => {
                if let Some(root) = &self.root {
                    let mut clipboard = Clipboard::new().unwrap();
                    clipboard
                        .set_text(root.to_string_recursive(0, true, &[]))
                        .unwrap();
                }
            }
            Message::FolderSelected(_) => {
                // フォルダ選択時の処理
            }
        }
    }

    // 再帰的にツリーを構築
    fn generate_tree_structure(&self) -> Result<TreeNode, std::io::Error> {
        let root_path = PathBuf::from(&self.folder_path);
        let root_node = generate_tree_recursive(&root_path)?;
        Ok(root_node)
    }
}

/// 再帰的にフォルダツリーを構築する関数
fn generate_tree_recursive(path: &PathBuf) -> std::io::Result<TreeNode> {
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

fn main() -> iced::Result {
    iced::run("TreeGen", TreeGen::update, TreeGen::view)
}
