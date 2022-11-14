use std::fs;

use color_eyre::Result;
use nucleoid_contributors::{models, templates};

fn main() -> Result<()> {
    let teams = models::load_teams()?;
    let people = models::load_people(&teams)?;
    let html = templates::test_page(people, &teams);
    fs::create_dir_all("build/")?;
    fs::write("build/index.html", html.0)?;
    Ok(())
}
