# Nucleoid Contributors!

This repo contains the list of all the Nucleoid contributors, which is displayed on the website.

## For contributors

### Updating your card

All the cards are stored in `data/people/`, and are in the TOML format with all lower-case names.

Here's everything you can include in your card:

```toml
# Name displayed at the top of the card
name = "Ashhhleyyy"

# (optional)
pronouns = "she/they"

# Group membership (don't change this)
groups = [
    "core",
    "moderator",
    "code",
]

# (optional)
bio = "Has only broken the server a few times."

# (optional) Overrides the Minecraft skin icon
avatar = "https://cdn.ashhhleyyy.dev/file/ashhhleyyy-assets/images/pfp.webp"

# Most of these are optional, bar minecraft
[socials]
# (required) Used for role sync later + used as fallback for avatar if not specified
minecraft = "5ad3ab57b55646359ba99a9a0568965a"

# (optional) Shown as username link
github = "ashhhleyyy"

# (optional)
# ID: maybe used for role-sync later
# Display: shown as-is
discord = { id = "828168927762841601", display = "Ash." }

# (optional) Used as a link icon
fediverse = "https://fedi.ashhhleyyy.dev/ash"
# (optional) Used as a link link
website = "https://ashhhleyyy.dev"
# (optional) Used as a link icon
twitter = "AshhhleyyyB"
```

If you're a member of the `NucleoidMC` GitHub organisation, you should have write permissions to this repository to update your card yourself. However, if you don't, either ask someone on Discord to invite you, or you just fork this repository and open a pull request with your changes.

**Please only update your card, and not anyone else's without their permission :)**

## Data access

The generated HTML and JSON files are hosted at https://contributors.nucleoid.xyz to be consumed by other things.

- `index.html` is a complete HTML page that can be opened in the browser and contains all styles embedded.
- `widget.html` is an HTML fragment that can be loaded into another page and themed, which is used [in the main Nucleoid website](https://github.com/NucleoidMC/nucleoid.xyz/blob/master/static/contributors.js)
- `data.json` contains all the data from the TOML files, including people and teams, serialised into one JSON file.
