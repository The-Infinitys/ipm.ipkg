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
|      |         | - package.sh
|      |         | - package/
|      |         |         | - purge.sh
|      |         |         | - remove.sh
|      |         |         | - install.sh
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