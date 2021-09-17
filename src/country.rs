use serde::Deserialize;
#[derive(Default, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct CountryDTO {
    pub name: String,
    pub region: String,
    pub area: Option<f32>,
    pub population: u64,
}

impl From<CountryDTO> for Country {
    fn from(dto: CountryDTO) -> Self {
        Self {
            area: dto.area.unwrap_or(0.0),
            name: dto.name,
            region: dto.region,
            population: dto.population,
        }
    }
}

#[derive(Default)]
pub struct Country {
    pub name: String,
    pub region: String,
    pub area: f32,
    pub population: u64,
}
