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


[mqtt]
host = "mqtt.acme-domain.tld"
port = 1883
topic = "spoolman/spool/state"
clientid = "SpoolMQ"
username = "mqttuser"
password = "super_secret"

[daemon]
stdout = ""                         # disabled logging if empty
stderr = "/tmp/spool-daemon.err"    # write stderr directed messages to file
pid_file = "/tmp/spool-daemon.pid"  # Pid file locations
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

## Daemon

Spool supports automatic updates for spool usage.
Based on the current weight of the whole spool, Spoolman will calculated the current filament left on spool.

This feature has been implemented as part of the [BBLPFilamentScale](https://github.com/jaypikay/BBLPFilamentScale) project, which is still in development, but in a proof-of-conecpt state.

The spool daemon will subscribe to the configured topic and expects a payload with the spool ID (_id_) and the current measured weight (_weight_).
These will than be sent to the API and update the remaining filament accordingly.

## Usage

### spool [help]

```
A Spoolman CLI helper

Usage: spool <COMMAND>

Commands:
  list        List available spools
  check       Check if a spool has enough available material
  use         Reduce the used filament in grams from the spool
  measured    Set measured weight for a spool
  daemon      Daemonize Mqtt monitor for automatic Spoolman updates
  completion  Completion generator
  help        Print this message or the help of the given subcommand(s)

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

### spool daemon

```
Daemonize Mqtt monitor for automatic Spoolman updates

Usage: spool daemon [OPTIONS]

Options:
  -f, --foreground  do not fork into background
  -h, --help        Print help
```

### spool completion

```
Completion generator

Usage: spool completion <SHELL>

Arguments:
  <SHELL>  Generate shell completion [possible values: bash, elvish, fish, powershell, zsh]

Options:
  -h, --help  Print help
```
