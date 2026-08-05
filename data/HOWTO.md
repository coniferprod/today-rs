# How to configure the Today program

The Today program can read information about historical events and observed days
from data sources in various formats. As of version 1.3, the supported formats
are:

* CSV
* Line-oriented text
* JSON from an HTTP(S) endpoint
* SQLite database
* XML conforming to the supplied XML Schema

This document contains details on how to prepare a CSV format datafile
and how to configure Today to use it as a data source.

## Example data file

The example data file is in CSV format, and contains information about
events related to the European Union: which countries joined (or exited)
the union, joined the Eurozone, or joined the Schengen agreement. The
information is collected from the European Union website.

The CSV data format is simple. There is no header row, and each line must
contain exactly three fields:
the date of the event, the description of the event, and the category of
the event. The formats of each field are as follows:

* Date: ISO 8601 format YYYY-MM-DD (without a time component)
* Description: free-format string, must be enclosed in double quotes if it
contains a comma (used as the CSV field delimiter)
* Category: free-format string. If it contains a slash, the part before the
slash is interpreted as the primary category, and the part after the slash
is the secondary category.

An example of a record in the CSV data file looks like this:

    1958-01-01,"Belgium, France, Germany, Italy and Netherlands join the EU",eu

If the date format is not valid ISO 8601, the whole line is discarded.

## Example configuration

The Today program is configured with a TOML format file called `today.toml`.
The program looks for this file in the appropriate directory specific to your operating
system. This lookup process is detailed in the _Learn Rust Programming Today_ book,
but just to recap, the directory is called `today`, and it is found here:

* Windows: `C:\Users\me\AppData\Roaming\today`
* macOS: `/Users/me/Library/Application Support/today`
* Linux: `/home/me/.config/today`

where `me` is your username. If you don't have this directory, the Today program
will try to create it in one of the locations above.

However, the Today program will not try to create the TOML format configuration
file (file a GitHub issue or make a pull request if you think it should!).
Therefore you will need to create it using a text editor.

For the CSV file example above, the contents of the configuration file are as
follows:

    [[providers]]
    name = "eu-events"
    kind = "csv"
    resource = "eu-events.csv"

The data files are also placed in the same directory as the configuration file.
The `resource` key is interpreted as relative to this directory. In this case,
just drop the `eu-events.csv` file into the configuration directory, and you
should be good to go.

## Testing the output

To test that your configuration and your data file work as expected, you can
make the Today program show events for a particular date using the `-d` or
`--date` option with the date in MMDD format. For example, to see the EU-related
events for January 1st, use the command

    today --date 0101

With the EU events data file in place and configured, you should a good number of
events listed, because most of them happened at the start of the year.

The `providers` subcommand should also list `eu-events` as one of the event
providers. If you want to temporarily disable this data source, set its
`is_active` key in the configuration file to `false`.
