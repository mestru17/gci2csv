# gci2csv
Converts json output from golangci-lint to csv.

## Motivation
golangci-lint can output its linter warnings/errors in a handful of formats. Its TSV format, however, is not formatted properly and tab characters are not
properly recognized by other tools, which makes it difficult to export to spreadsheets.

This tool helps alleviate that by accepting json formatted output from golangci-lint and then converts it to CSV which can easily be imported into spreadsheets.

## Running
Either grab the latest release from the Releases section or clone the repository and compile it from source yourself.
The executable from the Releases section is fully self contained.
