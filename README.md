# hq

[![ci](https://img.shields.io/github/actions/workflow/status/miller-time/hq/rust.yml)](https://github.com/miller-time/hq/actions/workflows/rust.yml)
[![crate](https://img.shields.io/crates/v/hq-rs)](https://crates.io/crates/hq-rs)
[![docs](https://img.shields.io/docsrs/hq-rs)](https://docs.rs/hq-rs/latest)

`hq` is a command-line HCL processor.

## install

This will install an `hq` binary on your system:

```sh
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

some_block "another_block_label" {
    attr = "another_value"
}
```

You can query the attribute(s) and block(s) in an HCL file like so:

```sh
$ cat example.hcl | hq '.some_attr'
```

```hcl
{
  foo = [
    1,
    2
  ]
  bar = true
}
```

```sh
$ cat example.hcl | hq '.some_attr.foo'
```

```hcl
[
  1,
  2
]
```

```sh
$ cat example.hcl | hq '.some_block'
```

```hcl
some_block "some_block_label" {
  attr = "value"
}
some_block "another_block_label" {
  attr = "another_value"
}
```

```sh
$ cat example.hcl | hq '.some_block[label="some_block_label"].attr'
```

```hcl
"value"
```

```sh
$ cat example.hcl | hq '.some_block[label="another_block_label"].attr'
```

```hcl
"another_value"
```

You can modify HCL (even HCL that is formatted and contains comments) like so:

```sh
$ cat example.hcl | hq write '.fmt_block.first_formatted_field' '"something_new"'
```

```hcl
some_attr = {
    foo = [1, 2]
    bar = true
}

some_block "some_block_label" {
    attr = "value"
}

some_block "another_block_label" {
    attr = "another_value"
}

# this is a block comment
fmt_block "fmt_label" {
    # this is a body comment
    # this is another body comment

    # this is a third body comment
    first_formatted_field  = "something_new"
    second_formatted_field = "second_value"
}
```
