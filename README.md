# Yew todo list mob programming

## 要件

### 概要
TODOアプリ
※ 登録=インメモリ

### やること

- [x] 登録してあるタスク（タスク名）を表示できる
  - [x] Structに状態としてタスクを保存
  - [x] 状態からタスクを取得し、HTMLを作成
- [ ] タスクを登録できる
- [ ] タスクに完了フラグを追加し、ボタン一つでタスクを完了にできる
- [ ] タスク名を編集できる

## 使用技術

- [yew](https://yew.rs/)

## 環境構築


### 必要ツール

- cargo
- trunk

### コマンド

```bash
trunk serve
```


### 参考
https://yew.rs/docs/getting-started/project-setup/using-trunk