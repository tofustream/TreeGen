use iced::widget::{button, checkbox, column, container, text, row};
use iced::{Alignment, Element, Length};
use std::fs;

#[derive(Default)]
struct FolderTree {
    folders: Vec<Folder>,
    show_hidden: bool,    // 隠しファイルの表示/非表示
}

#[derive(Debug, Clone)]
pub enum Message {
    ToggleHidden(bool),   // 隠しファイル表示切り替え
    RefreshTree,          // フォルダツリーのリフレッシュ
}

#[derive(Debug, Clone)]
struct Folder {
    name: String,
    files: Vec<String>,
}

impl FolderTree {
    // フォルダツリーを表示するためのビュー
    pub fn view(&self) -> Element<Message> {
        // フォルダリストを表示
        let folder_list = self.folders.iter().fold(column![], |col, folder| {
            col.push(
                column![
                    text(&folder.name).size(20),
                    folder.files.iter().fold(column![], |file_col, file| {
                        file_col.push(text(file))
                    }),
                ]
                .spacing(10),
            )
        });

        // UIコンテンツを構築
        let content = column![
            row![
                checkbox("Show hidden files", self.show_hidden)
                    .on_toggle(Message::ToggleHidden),
                button("Refresh Tree").on_press(Message::RefreshTree)
            ]
            .spacing(20),
            folder_list
        ]
        .spacing(20)
        .align_x(Alignment::Center);

        // コンテナで中央に配置
        container(content)
            .padding(10)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .into()
    }

    // メッセージに基づいて状態を更新
    pub fn update(&mut self, message: Message) {
        match message {
            Message::ToggleHidden(show) => {
                self.show_hidden = show;
                self.folders = generate_folder_tree(self.show_hidden); // フォルダツリーを更新
            }
            Message::RefreshTree => {
                self.folders = generate_folder_tree(self.show_hidden); // フォルダツリーを再生成
            }
        }
    }
}

// フォルダツリーを再帰的に生成
fn generate_folder_tree(show_hidden: bool) -> Vec<Folder> {
    let root = "."; // 現在のディレクトリ
    let mut folders = Vec::new();

    if let Ok(entries) = fs::read_dir(root) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    let folder_name = path.file_name().unwrap().to_str().unwrap().to_string();
                    if show_hidden || !folder_name.starts_with('.') {
                        let files = read_files_in_folder(&path);
                        folders.push(Folder {
                            name: folder_name,
                            files,
                        });
                    }
                }
            }
        }
    }
    folders
}

// フォルダ内のファイルを取得
fn read_files_in_folder(path: &std::path::Path) -> Vec<String> {
    let mut files = Vec::new();

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_name = entry.file_name().into_string().unwrap();
                files.push(file_name);
            }
        }
    }
    files
}

// メイン関数
fn main() -> iced::Result {
    iced::run("Folder Tree Generator", FolderTree::update, FolderTree::view)
}
