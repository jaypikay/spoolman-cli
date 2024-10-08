# A Spoolman CLI helper

## About

Spool is a CLI tool for [Spoolman](https://github.com/Donkie/Spoolman) to view, check and use spools.
Using a spool is meant to reduce the weight of a spool.
Whereas checking a spool helps identify spools with enough material left for a print.

## Configuration

### Configuration File

For configuration _spool_ uses a TOML file saved as `~/.config/spool/config.toml` to configure the Spoolman URL.

```toml
[spoolman]
url = "https://spoolman.acme-domain.tld"
```

### Environment Variables

#### http_proxy / https_proxy

If a proxy connection is required, the default environment variables can be used.

Default: ""

If a proxy connection is required.

Example:

```sh
export https_proxy=http://proxy.acme-domain.tld
```

## Usage

### spool

```
A Spoolman CLI helper

Usage: spool <COMMAND>

Commands:
  list      List available spools
  check     Check if a spool has enough available material
  use       Reduce the used filament in grams from the spool
  measured  Set measured weight for a spool
  help      Print this message or the help of the given subcommand(s)

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

### spool measured

```
Set measured weight for a spool

Usage: spool measured <SPOOLID> <WEIGHT>

Arguments:
  <SPOOLID>  Spool ID
  <WEIGHT>   Measured weight of spool in grams

Options:
  -h, --help  Print help
```
