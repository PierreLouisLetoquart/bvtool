# Binary visualization

> !!! This is a POC, I'm actually working on a more complete version of this project.

This algo aim to take in input any file types, read its raw binary data and generate in output a simple ppm img in P6 format which represent the file.

## Usage

Test if you have cargo installed :

```bash
cargo --version
```

Clone the repo and cd to the project :

```bash
git clone git@github.com:PierreLouisLetoquart/bvtool.git
cd bvtool
```

Run the project, it needs a file path as argument (the input file) :

```bash
cargo run -- target/debug/bvtool
```

We gave the binary itself as input, and the output is a ppm image. You can open it with any image viewer to see the result.

## TODO

- [x] fix map scaling values to [0, 255]
- [x] fix write ppm function
- [x] Tranfer draft doc to README
- [x] Generate a dataset of binary files representations
- [ ] add more file types support
- [ ] publish the dataset (on kaggle ?)
- [ ] add tests

## References

- [Christopher Domas - The future of RE Dynamic Binary Visualization](https://www.youtube.com/watch?v=4bM3Gut1hIk)
- [ppm P6 format](https://en.wikipedia.org/wiki/Netpbm) (for V0, now I'm using png format)
- [Corte.si - entropy in raw bin files](https://corte.si/posts/visualisation/entropy/)

## Datasets used to generate representaions

- [\[IMG\] Car](https://www.kaggle.com/datasets/prondeau/the-car-connection-picture-dataset)
- [\[IMG\] Cat + Faces](https://www.kaggle.com/datasets/prasunroy/natural-images)
- [\[IMG\] Bitmap](https://www.kaggle.com/datasets/tobiasbueck/bitmap-appple)
- [\[AUDIO\] Speech emotions -> .wav](https://www.kaggle.com/datasets/dmitrybabko/speech-emotion-recognition-en)
- \[EXEC\] All my executables in my $PATH dirs (ARM)
