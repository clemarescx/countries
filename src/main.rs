mod country;
mod language_table;
mod printer;
use country::Country;
use printer::Printer;
use std::path::PathBuf;
use std::{cmp::Ordering, error::Error};
use structopt::StructOpt;

use crate::{country::CountryDTO, language_table::LanguageTable};

const URL: &str = "https://restcountries.com/v2/all";

#[derive(StructOpt, Debug)]
enum SortBy {
    Name,
    Population,
    Area,
}

#[derive(StructOpt, Debug)]
struct SortOptions {
    #[structopt(subcommand)]
    sort_by: Option<SortBy>,
    #[structopt(short = "f", parse(from_os_str))]
    from_file: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let sort_options = match SortOptions::from_args_safe() {
        Ok(options) => options,
        Err(e) => {
            eprintln!("{}", e);
            return Ok(());
        }
    };

    let mut countries: Vec<Country> = if let Some(path) = &sort_options.from_file {
        read_countries(path).await
    } else {
        download_countries(&URL).await
    }
    .into_iter()
    .map(Country::from)
    .collect();

    sort_countries_by(&mut countries, &sort_options);

    Printer::print_table(&countries);
    println!();
    if let Err(e) = Printer::print_summary(&countries) {
        eprint!("error printing summary: {}", e)
    }
    let language_table = LanguageTable::from_countries(&countries);
    Printer::print_languages(&language_table);

    Ok(())
}

async fn read_countries(path: &PathBuf) -> Vec<CountryDTO> {
    let contents = tokio::fs::read_to_string(path)
        .await
        .expect("the file should be readable");

    serde_json::from_str(&contents).expect("the given json file to contain correct json")
}

async fn download_countries(url: &str) -> Vec<CountryDTO> {
    let client = reqwest::Client::builder()
        .build()
        .expect("the reqwest client should be built");

    println!("Querying {}", url);
    let response = client
        .get(url)
        .send()
        .await
        .expect("the GET request should be sent")
        .error_for_status()
        .expect("the GET request should be successful");

    response
        .json::<Vec<CountryDTO>>()
        .await
        .expect("the body of the response should deserialize to a list of countries")
}

fn sort_countries_by(countries: &mut [Country], options: &SortOptions) {
    if let Some(sort) = &options.sort_by {
        match sort {
            SortBy::Name => countries.sort_by_key(|c| c.name.clone()),
            SortBy::Population => countries.sort_by_key(|c| c.population),
            SortBy::Area => countries.sort_by(|a, b| {
                if a.area < b.area {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }),
        };
    }
}
