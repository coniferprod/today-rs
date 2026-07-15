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
events (see Chapter 31 of the LRPT book; the server is live on
[Heroku](https://todayserver-89bb2a1b2e80.herokuapp.com/api/v1/events)).

## Configuration file

The Today program can be configured with a file in the TOML format.
The LRPT book chapters 28 and 29 describe the format and usage in detail,
but here are some excerpts that should give an idea of it.

Make a TOML file called `today.toml` and put it in the place where
application config files are placed in your operating system (see the
documentation of the [dirs crate](https://crates.io/crates/dirs) for details).

A simple config file with just one entry looks like this:

    [[providers]]
    name = "history"
    kind = "csv"
    resource = "history.csv"

The `providers` sections list all the event providers that the program should
use. The `kind` key specifies the kind of event provider is being configured.

The `resource` key is used to locate a file that is needed by the provider, or to
supply a web address. Values that do not have a protocol prefix like `http://`
or `https://` should be taken as local files relative to the configuration directory,
whereas values that do have a protocol are treated as network resources.
The interpretation of the resource string will depend completely on the event
provider (CSV, text, SQLite, web).
