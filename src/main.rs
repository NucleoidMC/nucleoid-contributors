use std::{fs::{self, File}, collections::HashMap};

use color_eyre::Result;
use nucleoid_contributors::{models::{self, ContributorsData}, templates};

fn main() -> Result<()> {
    let teams = models::load_teams()?;
    let people = models::load_people(&teams)?;

    {
        let mut counts = HashMap::new();
        for person in people.values() {
            for group in &person.groups {
                *counts.entry(group.clone()).or_insert(0) += 1;
            }
        }
        println!("Loaded {count} people:", count = people.len());
        for (role, count) in counts {
            println!("  {role}: {count}");
        }
    }

    fs::create_dir_all("build/")?;

    {
        let mut f = File::create("build/data.json")?;
        serde_json::to_writer_pretty(&mut f, &ContributorsData {
            people: people.clone(),
            teams: teams.clone(),
        })?;
    }

    let mut people = people.into_values().collect::<Vec<_>>();
    people.sort_by_cached_key(|p| p.name.to_lowercase());
    people.sort_by_cached_key(|p| p.groups.iter()
        .filter_map(|t| teams.get(t))
        .map(|t| -t.weight)
        .min()
    );

    {
        let html = templates::test_page(&people, &teams);
        fs::write("build/index.html", html.0)?;
    }

    {
        let html = templates::widget(&people, &teams);
        fs::write("build/widget.html", html.0)?;
    }

    Ok(())
}
