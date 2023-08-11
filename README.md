# Schemas-builder

This crates is used to generate the `schemas` crate.

## Usage

Generate the `schemas` crate:

```bash
$ cargo run
```

Compile the `schemas` crate:

```bash
$ cd generated
$ cargo build
```

Each structure has its own cargo feature, disabled by default.  
Features match the name of the structure, e.g. `Action` or `CreativeWork`.  
Their properties are recursively enabled as well.

To see the full list of structures, run:

```bash
$ cargo doc --all-features --open
```
