# measure
Estimate data transfer speed between two streams

[![Build Status](https://travis-ci.com/mota/measure.svg?branch=master)](https://travis-ci.com/mota/measure)

## Usage
```
measure 1.0
Mota
Measures data transfer given in standard input

USAGE:
    measure [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --file <FILE>    File to output the transfer rate to instead of stderr
    -u, --unit <UNIT>    Display the result in a different unit format [possible values: b, k, m, g, t]
```

## Example

```
$ ssh me@my-host 'cat /dev/random' | measure > /dev/null
692.0 KiB/s
3.4 MiB/s
3.4 MiB/s
3.4 MiB/s
3.3 MiB/s
[...]
```

```
$ curl -s https://mirrors.sonic.net/pub/OpenBSD/6.4/amd64/install64.iso | measure -u k -f progress > install64.sio& # useless use of measure
$ tail -f progress
2452.0 KiB/s
5296.0 KiB/s
6688.0 KiB/s
6544.0 KiB/s
4176.0 KiB/s
6832.0 KiB/s
[...]
```
