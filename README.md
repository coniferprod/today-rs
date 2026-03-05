# today-rs

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
