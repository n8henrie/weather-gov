use std::{result,error};

type Error = Box<dyn error::Error + Send + Sync>;
type Result<T> = result::Result<T, Error>;

fn main() -> Result<()> {
    // Get grid by coordinates:
    // https://api.weather.gov/points/36.800934392484706,-108.69228949283561

    // https://api.weather.gov/gridpoints/ABQ/31,201/forecast
    let hourly_url = "https://api.weather.gov/gridpoints/ABQ/31,201/forecast/hourly";
    
    let resp = reqwest::blocking::get(hourly_url)?;
    println!("{}", resp.text()?);
    Ok(())
}