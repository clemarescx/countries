mod country;
mod printer;
use country::Country;
use printer::Printer;
use std::error::Error;

use crate::country::CountryDTO;

const URL: &str = "https://restcountries.eu/rest/v2/all";

struct SortOptions {}

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

    //     let sort_options = SortOptions {};
    //     sort_countries_by(&mut countries, &sort_options);

    Printer::print_table(&countries);
    println!();
    match Printer::print_summary(&countries) {
        Err(e) => eprint!("error printing summary: {}", e),
        _ => {}
    }
    Ok(())
}

fn sort_countries_by(countries: &mut [Country], options: &SortOptions) {
    todo!()
}
