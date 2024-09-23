use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum Message {
    FolderPathChanged(String),
    OpenFolderDialog,
    ApplyFilter, // フィルタリングを適用
    ToggleNodeCheck(Uuid, bool), // ノードのUUIDとチェック状態
    ShowFilterModal(bool), // フィルタモーダルの表示/非表示を切り替える
    CopyToClipboard,
}
