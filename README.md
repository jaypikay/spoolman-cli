# A Spoolman CLI helper

## About

Spool is a CLI tool for [Spoolman](https://github.com/Donkie/Spoolman) to view, check and use spools.
Using a spool is meant to reduce the weight of a spool.
Whereas checking a spool helps identify spools with enough material left for a print.

## Usage

### spool

```
A Spoolman CLI helper

Usage: spool <COMMAND>

Commands:
  list   List available spools
  check  Check if a spool has enough available material
  use    Reduce the used filament in grams from the spool
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

### spool list

```
List available spools

Usage: spool list

Options:
  -h, --help  Print help
```

### spool check

```
Check if a spool has enough available material

Usage: spool check <SPOOLID> <WEIGHT>

Arguments:
  <SPOOLID>  Spool ID
  <WEIGHT>   Used weight by print in grams

Options:
  -h, --help  Print help
```

### spool use

```
Reduce the used filament in grams from the spool

Usage: spool use <SPOOLID> <WEIGHT>

Arguments:
  <SPOOLID>  Spool ID
  <WEIGHT>   Used weight by print in grams

Options:
  -h, --help  Print help
```
