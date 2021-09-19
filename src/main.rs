mod country;
mod printer;
use country::Country;
use printer::Printer;
use std::{cmp::Ordering, error::Error};
use structopt::StructOpt;

use crate::country::CountryDTO;

const URL: &str = "https://restcountries.eu/rest/v2/all";

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
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::builder()
        .build()
        .expect("the reqwest client should be built");

    let response = client
        .get(URL)
        .send()
        .await
        .expect("the GET request should be sent")
        .error_for_status()
        .expect("the GET request should be successful");

    let mut countries: Vec<Country> = response
        .json::<Vec<CountryDTO>>()
        .await
        .expect("the body of the response should deserialize to a list of countries")
        .into_iter()
        .map(Country::from)
        .collect();

    let sort_options = match SortOptions::from_args_safe() {
        Ok(options) => options,
        Err(e) => {
            eprintln!("{}", e);
            panic!()
        }
    };
    sort_countries_by(&mut countries, &sort_options);

    Printer::print_table(&countries);
    println!();
    match Printer::print_summary(&countries) {
        Err(e) => eprint!("error printing summary: {}", e),
        _ => {}
    }
    Ok(())
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
