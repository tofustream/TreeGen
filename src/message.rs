#[derive(Debug, Clone)]
pub enum Message {
    FolderPathChanged(String),  // フォルダパスの変更
    GenerateTree,               // ツリー生成ボタンが押された
}
