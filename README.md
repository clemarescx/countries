# REST Countries

Built with Rust v1.55 stable

## Description

Simple CLI program used to print selected details from the [RESTCountry.eu API v2](https://restcountries.eu/rest/v2/all)

Prints:

- A table listing country details with columns:
  - _country name_ | _region_ | _area_ (in sq. miles) | _population_ in millions
- A summary showing:
  - average population per country (in millions)
  - country with biggest area in square miles
  - country with smallest area in square miles (non-null/zero values only)
- A table listing languages with countries and total population speaking it:
  - _languages_ | _countries_ | _population_ (in millions)

## Installation

Install Rust + Cargo with [Rustup](https://rustup.rs)

## Usage

Sorting is only implemented for the first table.

```shell
USAGE:
    countries.exe [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help          Prints this message or the help of the given subcommand(s)
    area
    name
    population
```

## Example

To print the countries table sorted by name, run: `cargo run -- name`
