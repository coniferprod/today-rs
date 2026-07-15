# today-rs

Today is a Rust application that shows you information about historical events
and days observed on the current date (or some other date you choose).

The development of the program is described in the book
[Learn Rust Programming Today](https://www.coniferproductions.com/books/learn-rust/) by Jere Käpyaho (Books on Demand, 2026).

This repository starts where the book ends. Over time, the program will be developed
further.

## Usage information

    Usage: today [OPTIONS] [COMMAND]

    Commands:
    providers  List all event providers
    add        Adds an event to an event provider
    help       Print this message or the help of the given subcommand(s)

    Options:
    -d, --date <DATE>        Event date in MMDD format
    -e, --exclude <EXCLUDE>  Categories to exclude, comma-separated (a/b,c/d)
    -h, --help               Print help


### Add subcommand

Adds an event to an event provider

    Usage: today add --provider-name <PROVIDER_NAME> --date <DATE> --description <DESCRIPTION> --category <CATEGORY>

    Options:
    -p, --provider-name <PROVIDER_NAME>  Name of event provider
    -d, --date <DATE>                    Date of event. Format: YYYY-MM-DD
    -e, --description <DESCRIPTION>      Description of event
    -c, --category <CATEGORY>            Category of event. Format: primary[/secondary]
    -h, --help                           Print help

### Providers subcommand

    List all event providers

    Usage: today providers

    Options:
    -h, --help  Print help

## Data files

The `data` directory contains some sample data files in various
formats. It is easy to make your own data files. There is also
a web service that can be used to test the program with some fake
events.

## Configuration file

Check out the LRPT book for more details on the configuration file
and its relation to the data formats. More information will be provided
here later.
