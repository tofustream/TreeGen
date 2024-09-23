use std::path::PathBuf;
use iced::widget::{
    button, column, container, scrollable, text, text_input, row, checkbox};
use iced::{Alignment, Element, Length};
use rfd::FileDialog;
use arboard::Clipboard;
use crate::message::Message;
use crate::tree::generate_tree_recursive;
use crate::tree::TreeNode;

#[derive(Default)]
pub struct TreeGen {
    folder_path: String,
    root: Option<TreeNode>, // フォルダツリー全体を保持
    show_filter_modal: bool, // フィルターモーダルを表示するかどうか
    filter_root: Option<TreeNode>, // フィルタリング用のツリー
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
        }
    }

    // 再帰的にツリーを構築
    fn generate_tree_structure(&self) -> Result<TreeNode, std::io::Error> {
        let root_path = PathBuf::from(&self.folder_path);
        let root_node = generate_tree_recursive(&root_path)?;
        Ok(root_node)
    }
}
