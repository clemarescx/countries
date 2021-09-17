use std::cmp::Ordering;

use crate::country::Country;

pub struct Printer;

impl Printer {
    pub fn print_table(countries: &[Country]) {
        let format_info = format_info(countries);
        let formatted_header = format_header(&format_info);
        // let table_header = format!("|{:^width$}|", "name", width = name_width + 1);
        let row_separator: String = "-".repeat(formatted_header.len());
        let formatted_countries = countries
            .into_iter()
            //     .map(|c| format!("| {:<width$}|", c.name, width = name_width));
            .map(|c| format_row(c, &format_info));
        println!("{}", row_separator);
        println!("{}", formatted_header);
        for row in formatted_countries {
            println!("{}", row_separator);
            println!("{}", row);
        }
        println!("{}", row_separator);
    }

    pub fn print_summary(countries: &[Country]) -> Result<(), String> {
        let average_pop = get_avg_population(countries);

        let Country {
            name: biggest_area_name,
            area: biggest_area,
            ..
        } = get_biggest_area(countries)
            .ok_or_else(|| "could not get country with biggest area".to_string())?;

        let Country {
            area: smallest_area,
            name: smallest_area_name,
            ..
        } = get_smallest_area(countries)
            .ok_or_else(|| "could not get country with smallest area".to_string())?;
        println!("Average population: {:.1} mil", average_pop / 1_000_000.0);
        println!(
            "Country with biggest area: {} ({:.0} sq mi)",
            biggest_area_name, biggest_area
        );
        println!(
            "Country with smallest area: {} ({} sq mi)",
            smallest_area_name, smallest_area
        );

        Ok(())
    }
}
fn get_avg_population(countries: &[Country]) -> f64 {
    countries.iter().map(|c| c.population as f64).sum::<f64>() / countries.len() as f64
}

fn get_biggest_area(countries: &[Country]) -> Option<&Country> {
    countries.iter().max_by(|a, b| {
        if a.area < b.area {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    })
}

fn get_smallest_area(countries: &[Country]) -> Option<&Country> {
    countries.iter().filter(|c| c.area > 0.0).min_by(|a, b| {
        if a.area < b.area {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    })
}
struct FormatInfo {
    name_width: usize,
    region_width: usize,
    area_width: usize,
    population_width: usize,
}

fn format_info(countries: &[Country]) -> FormatInfo {
    let [mut name_width, mut region_width, mut area_width, mut population_width]: [usize; 4] = [
        "name".len(),
        "region".len(),
        "area".len(),
        "population".len(),
    ];
    for Country {
        name,
        region,
        area,
        population,
    } in countries
    {
        name_width = name_width.max(name.len());
        region_width = region_width.max(region.len());
        area_width = area_width.max(format!("{:.0}", area).len());
        population_width = population_width.max(format!("{:.1}", population).len());
    }
    FormatInfo {
        name_width,
        region_width,
        area_width,
        population_width,
    }
}

fn format_header(format_info: &FormatInfo) -> String {
    let FormatInfo {
        name_width,
        region_width,
        area_width,
        population_width,
    } = format_info;
    format!(
        "|{0:^n_width$}|{1:^r_width$}|{2:^a_width$}|{3:^p_width$}|",
        "name",
        "region",
        "area",
        "population",
        n_width = name_width + 2,
        r_width = region_width + 2,
        a_width = area_width + 2,
        p_width = (population_width + 2).max("population".len()),
    )
}

fn format_row(country: &Country, format_info: &FormatInfo) -> String {
    let FormatInfo {
        name_width,
        region_width,
        area_width,
        population_width,
    } = format_info;
    format!(
        "| {0:<n_width$} | {1:<r_width$} | {2:<a_width$.0} | {3:<p_width$.1} |",
        country.name,
        country.region,
        country.area,
        country.population as f64 / 1_000_000.0,
        n_width = name_width,
        r_width = region_width,
        a_width = area_width,
        p_width = population_width,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_smallest_area() {
        let smallest_name = "smallest";
        let country_a = Country {
            name: smallest_name.to_string(),
            area: 0.1,
            ..Default::default()
        };
        let country_b = Country {
            area: 0.2,
            ..Default::default()
        };
        let countries = [country_a, country_b];
        let smallest = get_smallest_area(&countries);
        assert!(smallest.is_some());
        assert_eq!(smallest.unwrap().name, smallest_name);
    }

    #[test]
    fn find_biggest_area() {
        let biggest_name = "biggest";
        let country_a = Country {
            area: 0.1,
            ..Default::default()
        };
        let country_b = Country {
            name: biggest_name.to_string(),
            area: 0.2,
            ..Default::default()
        };
        let countries = [country_a, country_b];
        let biggest = get_biggest_area(&countries);
        assert!(biggest.is_some());
        assert_eq!(biggest.unwrap().name, biggest_name);
    }
}
