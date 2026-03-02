# aws

actionsからOIDCを使用してでawsのリソースにアクセスするためのメモ.

`IAM`と`IAM Identity Center`は違う. 5時間以上沼った.

## 基本

- overview: [docs](https://docs.github.com/en/actions/how-tos/secure-your-work/security-harden-deployments/oidc-in-aws)
- OIDCとは: [神記事](https://qiita.com/TakahikoKawasaki/items/498ca08bbfcc341691fe)
- IDプロバイダ作成: [docs](https://docs.aws.amazon.com/ja_jp/IAM/latest/UserGuide/id_roles_providers_create_oidc.html)
- assume: ~を引き受ける, ~の権限をまとう
  - assume role: roleを引き受ける (そのroleを引き受けてリソースにアクセスする)

- ポリシー: [docs](https://docs.aws.amazon.com/ja_jp/AmazonS3/latest/userguide/access-policy-language-overview.html)
- ポリシー詳細: [docs](https://docs.aws.amazon.com/ja_jp/AmazonS3/latest/userguide/example-bucket-policies.html)

補足

- githubのドキュメントにある`octo`は, `foo`, `bar`みたいなメタ構文変数
- `aud`とは`sub`は[qiita記事](https://qiita.com/TakahikoKawasaki/items/8f0e422c7edd2d220e06)読んで理解したうえで, githubがIDプロバイダとして発行される場合の値の形式は[docs](https://docs.github.com/ja/actions/concepts/security/openid-connect)見る

1. ロールを引き受ける: 何経由で引き受けられるか(^a)で設定
2. ロールを使ってアクセス: そのロールに対する認可を(^b)で設定

## 設定

1. ローカルから設定するための準備
2. ローカルから設定を反映

### 1. ローカルから設定するための準備

前提:

- `IAM Identity Center`の有効化

> 1つのアカウントで1つのリージョンまでしか有効化できない. 既にどこかのリージョンで有効化されていて, 無効化したいのに分からないってなったら[超神記事](https://repost.aws/questions/QUOWc-q2EKQDWindzFRWt2rA/iam-identity-center-already-registered-another-region)を見る.
> `CloudShell`はブラウザ上で実行できるので認証いらないので便利.

初期設定をローカルからできるようにssoで認証する.

```bash
aws configure sso
```

- デフォルト値あるやつはEnterでいい.
  - 特に`SSO registration scopes`はそのまま`Enter`
- これで色々頑張ったらブラウザで開いてユーザ登録, MFA登録などさせられる
- 終わったら自分のAWSアカウントにユーザと許可セットを紐づける
  - これめちゃ大事

ここまで来れたら下のコマンド叩いて, sso認証してもらうことで, ローカルで好きなコマンド叩ける.

```bash
aws sso login --profile <PROFILE>
```

### 2. ローカルから設定を反映

```bash
# ---------------------------------------------
# !    全部に`--profile <PROFILE>`つけること    !
# ---------------------------------------------
# `cat ~/.aws/config`したら上の変数名は確認できる


# 再接続したいとき
aws sso login --profile <PROFILE>

# 他にs3あるか確認
aws s3api head-bucket --bucket $BUCKET

# バケット作る
aws s3api create-bucket --bucket $BUCKET --region $REGION --create-bucket-configuration LocationConstraint=$REGION

# ロールあるか確認
aws iam get-role --role-name $ROLE_NAME

# ロール作る (^a)
aws iam create-role --role-name $ROLE_NAME --assume-role-policy-document file://<(./tools/aws/render-policy.sh ./tools/aws/trust.json)

# ポリシー付与 (^b)
aws iam put-role-policy --role-name $ROLE_NAME --policy-name $S3POLICY_NAME --policy-document file://<(./tools/aws/render-policy.sh ./tools/aws/s3-policy.json)

# `AWS_ROLE_TO_ASSUME`を取得
aws iam get-role --role-name "$ROLE_NAME" --query 'Role.Arn' --output text
```

## その他

- [AWS CLI docs](https://docs.aws.amazon.com/cli/latest): 見にくい
- WSLでは`~/.aws`に`/mnt/c/...`がシンボリックリンクされている. win側でAWSのクレデンシャルを登録し, コンテナに入らずにWSL側で叩くといいかも.
  - win, WSL側にAWS CLI必要でコンテナ側には不要
  - テストとかをコンテナ内でしたいならコンテナ側にもAWS CLI必要
  - [AWS CLIインストール](https://docs.aws.amazon.com/ja_jp/cli/latest/userguide/getting-started-install.html)
    - WSLでは`curl`で, winでは`winget`叩いた
