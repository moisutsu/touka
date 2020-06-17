# Touka

画像の背景を透過するコマンドラインツールです。

## インストール方法

```bash
git clone https://github.com/moisutsu/touka
cargo install --path touka
```

## 使い方

背景を透過したい画像をコマンドライン引数で与えるだけで、背景を透過した画像を`output.png`として出力します。

```bash
touka path/to/image
```

またオプションとして、画像の出力先のパス(-o)、白かどうかを判定するときに使うしきい値(-t)を指定できます。

```bash
touka -o out_image_name -t 250 path/to/image
```

画素値 r, g, bがすべてしきい値以上のとき、その画素を透過します。デフォルトのしきい値は230です。
