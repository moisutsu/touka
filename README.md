# Touka

画像の背景を透過するコマンドラインツールです。

This is a commandline tool for making image backgrounds transparent.

## インストール方法・Installation

```bash
cargo install touka
```

## 使い方・Usage

背景を透過したい画像をコマンドライン引数で与えるだけで、背景を透過した画像を
`output.png`として出力します。

Supply an image filepath as an argument to `touka` and new version of it named
`output.png` will be created with the background removed.

```bash
touka path/to/image
```

またオプションとして、画像の出力先のパス(-o)、白かどうかを判定するときに使うしき
い値(-t)を指定できます。

Use `-o` to supply a custom output name, and `-t` to control the background
detection threshold.

```bash
touka -o out_image_name -t 250 path/to/image
```

画素値 r, g, bがすべてしきい値以上のとき、その画素を透過します。デフォルトのしきい値は230です。

When all three RGB colour values are above the threshold, that pixel will be
made transparent. The default threshold is 230.
