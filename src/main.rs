use std::{fs, collections::HashMap};

use color_eyre::Result;
use nucleoid_contributors::{models, templates};

fn main() -> Result<()> {
    let teams = models::load_teams()?;
    let people = models::load_people(&teams)?;

    {
        let mut counts = HashMap::new();
        for person in &people {
            for group in &person.groups {
                *counts.entry(group.clone()).or_insert(0) += 1;
            }
        }
        println!("Loaded {count} people:", count = people.len());
        for (role, count) in counts {
            println!("  {role}: {count}");
        }
    }

    let html = templates::test_page(people, &teams);
    fs::create_dir_all("build/")?;
    fs::write("build/index.html", html.0)?;
    Ok(())
}
