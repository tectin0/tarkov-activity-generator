use anyhow::{Context, Result};

use rand::Rng;

#[derive(Debug, Clone, Default)]
pub(crate) struct RandomizeList(pub Vec<String>);

impl RandomizeList {
    pub(crate) fn random(&self) -> Result<String> {
        let length = self.0.len();

        if length == 0 {
            return Err(anyhow::anyhow!("Cannot randomize empty list"));
        }

        let mut rng = rand::thread_rng();

        let index = rng.gen_range(0..length);

        Ok(self.0[index].clone())
    }
}

pub(crate) fn get_weapons() -> Result<RandomizeList> {
    let response = reqwest::blocking::get("https://escapefromtarkov.fandom.com/wiki/Weapons")
        .context("Failed to get weapons page")?
        .text()
        .context("Failed to convert weapons page to text")?;

    let document = scraper::Html::parse_document(&response);

    let weapon_selector = scraper::Selector::parse("a.mw-redirect")
        .map_err(|_| anyhow::anyhow!("Failed to get weapon selector"))?;

    let weapons = document
        .select(&weapon_selector)
        .map(|x| {
            let html = x.html();

            let split = html.split("title=\"");

            let html = match split.last() {
                Some(x) => x,
                None => return String::new(),
            };

            let mut split = html.split('\"');

            let weapon = match split.next() {
                Some(x) => x,
                None => return String::new(),
            };

            weapon.replace("&quot;", r#"'"#)
        })
        .collect::<Vec<String>>();

    if weapons.iter().all(|x| x.is_empty()) {
        return Err(anyhow::anyhow!("Failed to get weapons"));
    }

    let cut_off_after = "NSV 'Utyos'";

    let weapons = weapons
        .into_iter()
        .take_while(|x| x != &cut_off_after.to_string())
        .collect::<Vec<String>>();

    Ok(RandomizeList(weapons))
}
