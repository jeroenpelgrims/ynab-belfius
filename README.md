# Belfius-ynab

This executable will convert a CSV export from the Belfius self-banking website to a format that can be understood by YNAB (You Need a Budget).
You can then immediately use the converted file to import the transactions into YNAB.

## How to use

1. Download the executable.

2. Run the convert command:  
   `ynab-belfius.exe /path/to/belfius-export.csv`  
   The file will then be converted and saved in the same folder as the original csv file.  
   The filename will be the same, but the extension will be `.ynab.csv`.  
   E.g.: The converted file for `/foo/bar/my-export.csv` will saved in this file: `/foo/bar/my-export.ynab.csv`  
   You can drag & drop this `*.ynab.csv` file straight into your transactions in YNAB.

## Building from source

You will need to have `cargo` installed.

1. Clone this repository.

2. Build the executable:  
   `cargo build --release`
