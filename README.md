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

Run the project, it needs a file path as argument for input and one for output :

```bash
cargo run -- target/debug/bvtool binvizual.ppm
```

We gave the binary itself as input, and the output is a ppm image named `binvizual.ppm`. You can open it with any image viewer to see the result.

## TODO

- [ ] fix map scaling values to [0, 255]
- [x] fix write ppm function
- [ ] Tranfer draft doc to README
- [ ] add tests

## References

- [Christopher Domas - The future of RE Dynamic Binary Visualization](https://www.youtube.com/watch?v=4bM3Gut1hIk)
- [ppm P6 format](https://en.wikipedia.org/wiki/Netpbm)
