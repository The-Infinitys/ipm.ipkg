
## コマンド
### 基本コマンド
- **help**  
  このヘルプメッセージを表示します。  
  使用例: `{name} help`

- **version**  
  {name} のバージョン情報を表示します。  
  使用例: `{name} version`

### project - プロジェクト管理
- **create**  
  新しいプロジェクトを作成します。  
  使用例: `{name} project create --project-name <名前> [--template <default|rust>]`

- **build**  
  プロジェクトをビルドします。  
  使用例: `{name} project build [--release] [--shell <bash|zsh>]`

- **install**  
  プロジェクトをインストールします。  
  使用例: `{name} project install [--global]`

- **remove**  
  プロジェクトを削除します。  
  使用例: `{name} project remove`

- **purge**  
  プロジェクトと関連データを完全に削除します。  
  使用例: `{name} project purge`

- **package**  
  プロジェクトをパッケージ化します。  
  使用例: `{name} project package`

- **metadata**  
  プロジェクトのメタデータを表示します。  
  使用例: `{name} project metadata`

### system - システム設定
- **configure**  
  ローカルまたはグローバル設定を構成します。  
  使用例: `{name} system configure [--local|--global]`

### package (pkg) - パッケージ管理
- **list**  
  インストール済みパッケージを表示します。  
  使用例: `{name} pkg list [--local|--global]`

## オプション
- `--help, -h`  
  ヘルプメッセージを表示します。
- `--version, -v`  
  バージョン情報を表示します。
- `--global`  
  グローバルモードで実行（デフォルト: ローカル）。
- `--shell <名前>`  
  使用するシェルを指定（例: bash, zsh）。
- `--release`  
  リリースモードでビルド。
- `--project-name <名前>`  
  プロジェクト名を指定。
- `--author-name <名前>`  
  作者名を指定。
- `--author-email <メール>`  
  作者のメールアドレスを指定。
- `--template <default|rust>`  
  プロジェクトテンプレートを指定。

## 詳細情報
{name} は、プロジェクトの作成、ビルド、インストールを簡単に行えるツールです。  
ローカル（~/.ipkg）またはグローバル（/etc/ipkg）でのパッケージ管理をサポートし、依存関係の解決やバージョン管理も可能です。  
さらに詳しい情報は、公式ドキュメントを参照してください。
