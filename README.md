# Streamgen

Streamgen is a CLI tool for generating streams of data for testing stream processing applications with
engines like [Arroyo](https://github.com/ArroyoSystems/arroyo) or [Apache Flink](https://flink.apache.org/).

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
[![Build Status][actions-badge]][actions-url]

[crates-badge]: https://img.shields.io/crates/v/streamgen.svg
[crates-url]: https://crates.io/crates/streamgen
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/ArroyoSystems/streamgen/blob/master/LICENSE-MIT
[actions-badge]: https://github.com/ArroyoSystems/streamgen/actions/workflows/ci.yml/badge.svg
[actions-url]: https://github.com/ArroyoSystems/streamgen/actions?query=branch%3Amain


## Features

### Sinks

Streamgen can expose generated data via the following sinks:

* `stdout` - Write data to stdout
* `sse` - Run a [Server-Sent Events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events) server
* `kafka` - Write data to a Kafka topic

### Formats

* `string`
* `json`

### Generators

* `common-log` - [Common Log Format](https://en.wikipedia.org/wiki/Common_Log_Format) records
* `impulse` - Stream of incrementing integers
* `order` - Simulated web order events
* `stock-trade` - Simulated stock trades

## Usage

```
$ streamgen --help

A tool for generating streams of data for testing and benchmarking.


Usage: streamgen [OPTIONS] <SPEC> [COMMAND]

Commands:
  stdout  Write outputs to stdout
  sse     Run a Server-Sent Events server
  kafka   Write outputs to Kafka
  help    Print this message or the help of the given subcommand(s)

Arguments:
  <SPEC>  Type of data to generator [possible values: common-log, impulse, order, stock-trade]

Options:
  -f, --format <FORMAT>  Format of the generated data [possible values: string, json]
  -r, --rate <RATE>      Rate of generation in records per second
  -l, --limit <LIMIT>    Max number of records to generate
  -h, --help             Print help
  -V, --version          Print version
```

Writing to stdout:
```
$ streamgen --rate 10 --format string common-log stdout
66.70.249.106 - travis_quo [01/Dec/2023:14:52:26 -0800] "GET /company.doc" 405 4521 "-" "Mozilla/5.0 (Windows NT 6.1) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/41.0.2228.0 Safari/537.36"
43.89.162.120 - vesta_itaque [01/Dec/2023:14:52:26 -0800] "GET /tmp/first/same.doc" 400 4767 "-" "Mozilla/5.0 (iPhone; U; CPU iPhone OS 4_2_1 like Mac OS X; nb-no) AppleWebKit/533.17.9 (KHTML, like Gecko) Version/5.0.2 Mobile/8C148a Safari/6533.18.5"
206.157.23.40 - lulu_molestias [01/Dec/2023:14:52:27 -0800] "GET /sbin/charlotte.rar" 401 1519 "-" "Mozilla/5.0 (compatible; MSIE 8.0; Windows NT 5.1; Trident/4.0; .NET CLR 1.1.4322; .NET CLR 2.0.50727)"
63.37.55.188 - roscoe_nemo [01/Dec/2023:14:52:27 -0800] "POST /var/problem/government.xls" 400 7044 "-" "Mozilla/5.0 (iPad; CPU OS 5_1 like Mac OS X) AppleWebKit/534.46 (KHTML, like Gecko ) Version/5.1 Mobile/9B176 Safari/7534.48.3"
129.215.157.150 - angie_dolorum [01/Dec/2023:14:52:27 -0800] "GET /usr.txt" 405 2768 "-" "Mozilla/5.0 (iPad; CPU OS 5_1 like Mac OS X) AppleWebKit/534.46 (KHTML, like Gecko ) Version/5.1 Mobile/9B176 Safari/7534.48.3"
213.73.92.8 - roscoe_omnis [01/Dec/2023:14:52:27 -0800] "GET /etc/case/life.ppt" 404 4371 "-" "Opera/9.80 (Windows NT 6.0) Presto/2.12.388 Version/12.14"
```

Writing to Kafka
```
$ streamgen --format json order kafka --topic orders --boostrap-servers localhost:9092
```

## Installation

### From binaries

Pre-built binaries are available for Linux and macOS on the 
[releases page](https://github.com/ArroyoSystems/streamgen/releases).

### From source

Streamgen can be built from source via Cargo:

```
$ cargo install streamgen
```

If you would like Kafka support, you will need to pass the `--features kafka` flag to Cargo:

```
$ cargo install streamgen --features kafka
```

This relies on the rust-rdkafka library; to use this you will need to have the necessary dependencies installed
on your system. See the [rust-rdkafka README](https://github.com/fede1024/rust-rdkafka#installation) for more details.