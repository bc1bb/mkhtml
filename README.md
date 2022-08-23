# mkhtml
Makes HTML files from `header.html` and `footer.html` and `pages`.

## Installation
```shell
cargo install mkhtml
```

## Building
```shell
cargo build     # dev
cargo build -r  # release
```

## How to setup
- put your header in `parts/header.html`,
- put your footer in `parts/footer.html`,
- put your pages in `pages/` (can have folders),
- `mkhtml build`. (`b` also works).

## Arguments
By default `mkhtml` will build in the working directory but you can change that by using any of the following arguments:

- `--pages-dir [path]`,
- `--parts-dir [path]`,
- `--static-dir [path]`,
- `--build-dir [path]`.

(you can use one or more of them, you can use both absolute and relative paths).