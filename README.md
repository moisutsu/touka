# Touka

画像の背景を透過するコマンドラインツールです。

This is a commandline tool for making image backgrounds transparent.

## インストール方法・Installation

```bash
cargo install touka
```

## 使い方・Usage

背景を透過したい画像と出力先のパスを`touka`のコマンドライン引数で与えるだけで、背景を透過した画像をファイルとして出力します。

You can make a transparent image with a transparent background by passing the path of the image and the output path to the destination as a command line argument of `touka`.

```bash
touka input_image_path -o output_image_path
```

---

またオプションとして、`-t`を使うことで白かどうかを判定するときに使うしきい値を指定できます。

Use `-t` to control the background detection threshold.

```bash
touka input_image_path -o output_image_path -t 250
```

画素値 r, g, bがすべてしきい値以上のとき、その画素を透過します。デフォルトのしきい値は230です。

When all three RGB colour values are above the threshold, that pixel will be
made transparent. The default threshold is 230.

---

macOS限定で直接クリップボードと画像をやり取りすることができます。入力画像を指定しない場合はクリップボードの画像を透過し、出力パスを指定しない場合はクリップボードに透過した画像を保存します。

Only for macOS, you can directly exchange images with the clipboard. When you don't specify an input image, the clipboard image is transparent, and when you don't specify an output path, the transparent image is saved to the clipboard.

```bash
# The clipboard image is transparent and saved to the clipboard.
touka
```
