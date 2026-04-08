# rpx2json（RPX2ACR）

RPX（ActiveReports）ファイルを ACR（JSON）形式へ変換するツールです。
※ Windows専用

---

## English

English version → README.md

---

## 概要

rpx2json は、ActiveReports で作成された RPX（帳票定義）を
ACR（Across Report）で利用可能な JSON 形式（帳票定義）へ変換するコマンドラインツールです。

既存の帳票資産をそのまま活用しながら、ACR環境への移行を支援します。

---

## 対象ユーザー

* ActiveReports を利用している開発者
* 帳票システムの移行・再構築を検討している方
* ACR（Across Report）を利用する開発者

---

## 動作環境

* Windows 10 / 11
* ActiveReports により生成された .rpx ファイル

---

## ダウンロード

以下より実行ファイルを取得できます：

https://github.com/acrossreport/rpx2json/releases

---

## 使い方

コマンドプロンプトから実行します。

```bash
rpx2json.exe sample.rpx
```

---

## 出力

変換結果は JSON 形式で標準出力に出力されます。

---

## 対応内容

* 帳票レイアウト構造の変換
* セクション情報の変換
* コントロール情報の変換

---

## 制限事項

* 一部の表現・レイアウトは完全再現されない場合があります
* スクリプトや特殊な機能は対象外です

---

## 設計方針

* 既存資産（RPX）を活用した移行を重視
* JSONベースのシンプルな構造
* プラットフォームに依存しない帳票定義

---

## 関連

* ACR（Across Report）
* ActiveReports

---

## ライセンス

本リポジトリは公開版です。
内部実装は別リポジトリにて管理されています。
