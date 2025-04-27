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
