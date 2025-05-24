# About Infinite Package Project

## Scripts

`ipkg/scripts` ディレクトリに入っているスクリプトファイルは、
パッケージを操作する際に使用するものです。

### 実行

すべてのシェルスクリプトは、プロジェクトのルートディレクトリで行われます。
つまり、project.yamlが存在するフォルダがカレントディレクトリになるということです。
 /
| - ipkg/
|      | - scripts/
|      |         | - install.sh
|      |         | - remove.sh
|      |         | - build.sh
|      |         | - purge.sh
|      | - project-ignore.yaml
| - project.yaml
| - src/
      | ...


### 環境変数

#### 共通
 - $IPKG_PROJECT_NAME :
    Infinite Packageのパッケージ名が代入されています。

 - $IPKG_PROJECT_VERSION :
    Infinite Packageのバージョンが代入されています。

#### 特有
 - $IPKG_BUILD_MODE :
    Infinite Packageをビルドする際のモードが代入されています。
        値: release / debug

 - $IPKG_INSTALL_MODE | $IPKG_REMOVE_MODE | $IPKG_PURGE_MODE :
    Infinite Packageを管理(インストール・削除・パージ)する際のモードが代入されています。
        値: local / global

### パッケージング

`ipkg project package`
を実行することによって、プロジェクトをパッケージにすることができます。

```yaml
source-build:
  - log/
normal:
  - .gitignore
  - .git
min:
  - src/
  - target/debug/
  - target/release/*/
  - target/release/*.d
  - target/release/*rlib
  - Cargo*
```

project-ignore.yamlに無視するファイルやフォルダのデータを入れることができます。
