# hq

[![crate](https://img.shields.io/crates/v/hq-rs)](https://crates.io/crates/hq-rs)
[![docs](https://img.shields.io/docsrs/hq-rs)](https://docs.rs/hq-rs/latest)

`hq` is a command-line HCL processor.

## install

This will install an `hq` binary on your system:

```
$ cargo install hq-rs
```

## usage

Here is an example HCL file:

```hcl
some_attr = {
    foo = [1, 2]
    bar = true
}

some_block "some_block_label" {
    attr = "value"
}
```

You can query the attribute(s) and block(s) in an HCL file like so:

```sh
$ cat example.hcl | hq '.some_attr'
{
  foo = [
    1,
    2
  ]
  bar = true
}

$ cat example.hcl | hq '.some_attr.foo'
[
  1,
  2
]

$ cat example.hcl | hq '.some_block'
attr = "value"

$ cat example.hcl | hq '.some_block.attr'
"value"
```
