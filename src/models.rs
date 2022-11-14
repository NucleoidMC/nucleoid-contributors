use std::{fs, path::Path, borrow::Cow, collections::HashMap};

use color_eyre::{Result, eyre::eyre};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Person {
    pub name: String,
    pub pronouns: Option<String>,
    pub groups: Vec<String>,
    pub bio: Option<String>,
    pub avatar: Option<String>,
    pub socials: Socials,
}

impl Person {
    pub fn avatar(&self) -> Cow<str> {
        if let Some(avatar) = &self.avatar {
            Cow::Borrowed(&avatar)
        } else {
            Cow::Owned(format!("https://api.nucleoid.xyz/skin/face/128/{uuid}", uuid = self.socials.minecraft))
        }
    }
}

#[derive(Deserialize)]
pub struct Socials {
    pub github: Option<String>,
    pub discord: Option<Discord>,
    pub minecraft: String,
    pub fediverse: Option<String>,
    pub website: Option<String>,
    pub twitter: Option<String>,
}

#[derive(Deserialize)]
pub struct Discord {
    pub id: String,
    pub display: String,
}

#[derive(Deserialize)]
pub struct Team {
    pub name: String,
    pub colour: Option<String>,
    #[serde(default)]
    pub weight: i32,
}

pub fn load_people(teams: &HashMap<String, Team>) -> Result<Vec<Person>> {
    let mut people = Vec::new();

    for entry in fs::read_dir("data/people/")? {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            people.push(load_person(&entry.path())?);
        }
    }

    people.sort_by_cached_key(|p| p.name.to_lowercase());
    people.sort_by_cached_key(|p| p.groups.iter()
        .filter_map(|t| teams.get(t))
        .map(|t| -t.weight)
        .min()
    );

    Ok(people)
}

pub fn load_teams() -> Result<HashMap<String, Team>> {
    let mut teams = HashMap::new();

    for entry in fs::read_dir("data/teams/")? {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            let path = entry.path();
            let name = path.file_stem().expect("file to have a name").to_string_lossy().to_string();
            teams.insert(name, load_team(&path)?);
        }
    }

    Ok(teams)
}

fn load_person(path: &Path) -> Result<Person> {
    let data = fs::read_to_string(path)?;
    toml::from_str(&data).map_err(|e| eyre!("failed to parse {path:?}: {e}"))
}

fn load_team(path: &Path) -> Result<Team> {
    let data = fs::read_to_string(path)?;
    toml::from_str(&data).map_err(|e| eyre!("failed to parse {path:?}: {e}"))
}
