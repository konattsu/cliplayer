# aws

## OIDC

### brief

- [docs](https://docs.github.com/en/actions/how-tos/secure-your-work/security-harden-deployments/oidc-in-aws)に沿って進めればok
- [OIDCとは](https://qiita.com/TakahikoKawasaki/items/498ca08bbfcc341691fe): 神記事
- [IDプロバイダ作成](https://docs.aws.amazon.com/ja_jp/IAM/latest/UserGuide/id_roles_providers_create_oidc.html): 一番最初に必要

### 個人的理解メモ

- githubのドキュメントにある`octo`は, `foo`, `bar`みたいなメタ構文変数
- `aud`とは`sub`は[qiita記事](https://qiita.com/TakahikoKawasaki/items/8f0e422c7edd2d220e06)読んで理解したうえで, githubがIDプロバイダとして発行される場合の値の形式は[docs](https://docs.github.com/ja/actions/concepts/security/openid-connect)見る

## S3

- [ポリシー](https://docs.aws.amazon.com/ja_jp/AmazonS3/latest/userguide/access-policy-language-overview.html)
- [ポリシー詳細](https://docs.aws.amazon.com/ja_jp/AmazonS3/latest/userguide/example-bucket-policies.html)

## その他

- [AWS CLI](https://docs.aws.amazon.com/cli/latest): 見にくい
- WSLでは`~/.aws`に`/mnt/c/...`がシンボリックリンクされている. win側でAWSのクレデンシャルを登録し, コンテナに入らずにWSL側で叩くといいかも.
  - win, WSL側にAWS CLI必要でコンテナ側には不要
  - テストとかをコンテナ内でしたいならコンテナ側にもAWS CLI必要
  - [AWS CLIインストール](https://docs.aws.amazon.com/ja_jp/cli/latest/userguide/getting-started-install.html)
    - WSLでは`curl`で, winでは`winget`叩いた
