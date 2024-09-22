use iced::widget::{button, column, container, scrollable, text, text_input};
use iced::{Alignment, Element, Length};
use crate::message::Message;
use crate::tree::generate_tree_structure;

#[derive(Default)]
pub struct TreeGen {
    pub folder_path: String,      // ユーザーが入力したフォルダパス
    pub tree_structure: String,   // 生成されたフォルダツリー
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
