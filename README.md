# dtsfmt

Auto formatter for device tree files.

## Installation

```bash
git clone --recurse-submodules https://github.com/juliamertz/dtsfmt.git
cd dtsfmt
cargo install --path .
```

## Usage

To run dtsfmt, simply provide a file/directory path to the `dtsfmt` command.

```bash
dtsfmt .
```

## Config

The following configuration options are available for dtsfmt. Configuration should
be added to a `.dtsfmtrc.toml` file at the root of your project.

```toml
layout = "kinesis:adv360" # required

[options]
separate_sections = false # print newline between sections
indent_size = 2
tabs = true
```

## Ignoring code

You can add a `.dtsfmtignore` file at the root of your project to exclude files
and paths from formatting. This file follows the same rules as `.gitignore`.

To ignore specific code blocks add a comment starting with `dtsfmt-ignore`,
this will skip formatting for the next node.

## Flags

### `--check`

When you want to check if your files are formatted, you can run dtsfmt with
the `--check` flag (or `-c`). This will output a human-friendly message and a
list of unformatted files, if any.

```bash
dtsfmt --check .
```

### `--emit`

You can change the way dtsfmt emits the changes with the `--emit` flag.

```bash
dtsfmt --emit=stdout
```

### `--config-file`

Instead of playing your configuration file at your project directory's root,
you can pass a file path with `--config-file <path>` which will be used instead
