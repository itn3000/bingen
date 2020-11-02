# Binary data generator

This is the tool for generating binary data file.

# Usage

you can get help by `bingen --help`
`-c <COUNT>` must be passed, and one of the following data source option must be passed;

* `-b <BASE64>`
    * data source from base64 string
* `-f <FILE_PATH>`
    * data source from regular file
* `-x <HEX_STRING>`
    * data source from hex string
* `-i <MAX_LENGTH>`
    * data source from standard input(max length)
* `-s <INPUT_STRING>`
    * data source from utf8 string
* `-r`
    * data source from random byte generator

and following additional options;

* `-o <OUTPUT_FILE_PATH>`
    * outpu data file path(default: stdout)
* `-d <HEX_STRING>`
    * hex string for delimiting data(default: no delimiter)

# Example

## pass data by normal string and output to stdout 10 times, delimited by CRLF

```
> bingen -s "All work and no play makes Jack a dull boy" -c 10 -d 0d0a
All work and no play makes Jack a dull boy
All work and no play makes Jack a dull boy
All work and no play makes Jack a dull boy
All work and no play makes Jack a dull boy
All work and no play makes Jack a dull boy
All work and no play makes Jack a dull boy
All work and no play makes Jack a dull boy
All work and no play makes Jack a dull boy
All work and no play makes Jack a dull boy
All work and no play makes Jack a dull boy
```

## generate 1024 bytes data randomly and output file to x.bin

```
> bingen -r -c 1024 -o x.bin
```
