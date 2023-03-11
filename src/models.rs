use std::{fs, path::Path, borrow::Cow, collections::{HashMap, BTreeMap}};

use color_eyre::{Result, eyre::eyre};
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct Person {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pronouns: Option<String>,
    pub groups: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
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

#[derive(Clone, Deserialize, Serialize)]
pub struct Socials {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub github: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discord: Option<Discord>,
    pub minecraft: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fediverse: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitter: Option<String>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Discord {
    pub id: String,
    pub display: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Team {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colour: Option<String>,
    #[serde(default)]
    pub weight: i32,
}

#[derive(Serialize)]
pub struct ContributorsData {
    pub people: BTreeMap<String, Person>,
    pub teams: BTreeMap<String, Team>,
}

// Uses a BTreeMap to ensure consistent ordering of the keys in the output JSON,
// and we don't care about the performance costs
pub fn load_people(teams: &BTreeMap<String, Team>) -> Result<BTreeMap<String, Person>> {
    let mut people = BTreeMap::new();

    for entry in fs::read_dir("data/people/")? {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            let name = entry.path().file_stem().unwrap().to_str().expect("contributor file name to be valid UTF-8").to_owned();
            people.insert(name, load_person(&entry.path())?);
        }
    }

    Ok(people)
}

pub fn load_teams() -> Result<BTreeMap<String, Team>> {
    let mut teams = BTreeMap::new();

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
