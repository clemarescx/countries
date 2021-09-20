use std::collections::HashMap;

use crate::country::Country;

pub struct LanguageTable {
    pub countries_column_width: usize,
    pub languages_column_width: usize,
    pub population_column_width: usize,
    pub languages: HashMap<String, Vec<(String, u64)>>,
}

impl LanguageTable {
    pub fn from_countries(countries: &[Country]) -> Self {
        let mut countries_column_width = "countries".len();
        let mut languages_column_width = "languages".len();
        let population_column_width = "population".len();

        let mut languages: HashMap<String, Vec<(String, u64)>> = HashMap::new();
        for country in countries {
            let country_languages = &country.languages;
            for language in country_languages {
                languages
                    .entry(language.name.clone())
                    .or_insert_with(Vec::new)
                    .push((country.name.clone(), country.population));
                languages_column_width = languages_column_width.max(language.name.len());
            }

            countries_column_width = countries_column_width.max(country.name.len());
        }
        LanguageTable {
            countries_column_width,
            languages_column_width,
            population_column_width,
            languages,
        }
    }
}
