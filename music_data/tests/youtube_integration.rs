// send_request を mock して統合テスト/単体テストを行う手順

// 1. send_request を trait で抽象化する
//    例: trait YouTubeApiRequest { async fn send_request(&self, url: &str) -> ... }
//    本体は YouTubeApiRequest を実装し、テスト時はモック実装を使う

// 2. テスト時はモック実装を注入する
//    - モック実装では、リクエスト内容に応じて任意のレスポンスやエラーを返す
//    - これにより外部APIに依存せず、様々なケースをテストできる

// 3. テストケース例
//    - 正常系: 200 OK で期待通りの JSON を返す
//    - 異常系: 403 Forbidden, 404 Not Found, ネットワークエラー, パースエラー など
//    - レスポンス内容による分岐: items が空、部分的に存在しないID など

// 4. テストフレームワーク
//    - async_trait + mockall などで trait の async 関数をモック
//    - もしくは send_request を pub(crate) にして #[cfg(test)] で差し替え

// 5. 既存の統合テストのように、APIサーバを mockito などで立ててリクエストを受ける方法も有効
//    - ただし trait 化によるモックの方が細かい制御ができる

// まとめ: send_request を trait で抽象化し、テスト時はモック実装を注入することで、外部APIに依存しない柔軟なテストが可能です。
