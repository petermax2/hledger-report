# hledger Report
## Abstract

`hledger-report` is a template-based report generator for the [hledger](https://hledger.org/) plain text accounting software.

While `hledger` offers powerful command-line reporting capabilities and supports CSV, TSV, and JSON output formats, it lacks built-in functionality for generating PDF reports. `hledger-report` addresses this gap by transforming `hledger` output into professionally formatted documents using Jinja-like templates.

### Key Features

- Transforms `hledger` data into typeset documents
- Uses [minijinja](https://crates.io/crates/minijinja) as the template engine
- Most likely used output formats for PDF generation:
  - [LaTeX](https://en.wikipedia.org/wiki/LaTeX)
  - [Typst](https://typst.app/)

### Why hledger-report?

Perfect for users who need professional financial reports from their `hledger` data without manual formatting.

## Configuration
### Format

The configuration file format is TOML.

### Location

The configuration file path can be provided per environment variable `HLEDGER_REPORT_CONFIG`.

If the environment variable is not set, the default location `$HOME/.config/hledger-report/config.toml` is being used.

The configuration file must be provided and is mandatory for correct `hledger-report` execution.

### Options

#### hledger

The section `[hledger]` configures the interaction with the `hledger` process.

| **field** | **description**                    | **default value** |
|-----------|------------------------------------|-------------------|
| path      | the path of the `hledger` binary   | `hledger`         |

The following example demonstrates how to configure the `[hledger]` section:

```toml
[hledger]
path = "/usr/local/bin/hledger"
```
