# Touka

[English](./README.md)

画像の背景を透過するコマンドラインツールです。

## インストール方法

```bash
cargo install touka
```

## 使い方

背景を透過したい画像と出力先のパスを`touka`のコマンドライン引数で与えるだけで、背景を透過した画像をファイルとして出力します。

```bash
touka input/image/path -o output/image/path
```

またオプションとして、`-t`を使うことで白かどうかを判定するときに使うしきい値を指定できます。

```bash
touka input/image/path -o output/image/path -t 250
```

画素値 r, g, bがすべてしきい値以上のとき、その画素を透過します。デフォルトのしきい値は230です。

macOS限定で直接クリップボードと画像をやり取りすることができます。入力画像を指定しない場合はクリップボードの画像を透過し、出力パスを指定しない場合はクリップボードに透過した画像を保存します。

```bash
# The clipboard image's background is transparent and saved to the clipboard.
touka
```
