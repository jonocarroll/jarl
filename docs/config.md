# Configuring Jarl

## With the command line

Jarl comes with various options available directly from the command line.
These can be listed with `jarl check --help`:

```sh
Check a set of files or directories

Usage: jarl check [OPTIONS] <FILES>...

Arguments:
  <FILES>...
          List of files or directories to check or fix lints, for example `jarl check .`.

Options:
  -f, --fix
          Automatically fix issues detected by the linter.

  -u, --unsafe-fixes
          Include fixes that may not retain the original intent of the  code.

      --fix-only
          Apply fixes to resolve lint violations, but don't report on leftover violations. Implies `--fix`.

      --allow-dirty
          Apply fixes even if the Git branch is not clean, meaning that there are uncommitted files.

      --allow-no-vcs
          Apply fixes even if there is no version control system.

  -s, --select-rules <SELECT_RULES>
          Names of rules to include, separated by a comma (no spaces). This also accepts names of groups of rules, such as "PERF".

          [default: ]

  -i, --ignore-rules <IGNORE_RULES>
          Names of rules to exclude, separated by a comma (no spaces). This also accepts names of groups of rules, such as "PERF".

          [default: ]

  -w, --with-timing
          Show the time taken by the function.

  -m, --min-r-version <MIN_R_VERSION>
          The mimimum R version to be used by the linter. Some rules only work starting from a specific version.

      --output-format <OUTPUT_FORMAT>
          Output serialization format for violations.

          Possible values:
          - full:    Print diagnostics with full context using annotated code snippets
          - concise: Print diagnostics in a concise format, one per line
          - github:  Print diagnostics as GitHub format
          - json:    Print diagnostics as JSON

          [default: full]

      --assignment-op <ASSIGNMENT_OP>
          Assignment operator to use, can be either `<-` or `=`.

  -h, --help
          Print help (see a summary with '-h')
```

You can pass multiple options at once, for instance

```sh
jarl check . --fix --select-rules any_is_na,class_equals
```

## With a config file

To avoid typing options every time and to ensure all uses of Jarl in a project are consistent, it is possible to store options in `jarl.toml`.

For now, this only supports two fields: `select` and `ignore` to determine which rules to use.
This file looks like this:

```toml
[lint]
select = []
ignore = []
```

This has the same capabilities as `--select-rules` and `--ignore-rules`, so it is possible to pass rule names and names of groups of rules:

```toml
[lint]
select = ["PERF", "length_test"]
ignore = ["SUSP"]
```

::: {.callout-note}
## Using CLI arguments and `jarl.toml`

Arguments in the command line always have the priority on those specified in `jarl.toml`.
For example, if you have the following file:
```toml
[lint]
select = ["PERF", "length_test"]
ignore = []
```
then calling
```sh
jarl check . --ignore-rules PERF
```

will only apply the rule `length_test`.
:::
