use reqwest;
pub use reqwest::Error;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[cfg(test)]
use mockito;

pub async fn get_url<T>(url: &str) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    let resp = reqwest::get(url).await?.json::<T>().await?;
    Ok(resp)
}

#[derive(Deserialize, Serialize, Debug, Clone, Default, PartialEq)]
pub struct PageResponse<T> {
    pub results: Vec<T>,

    pub info: Info,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default, PartialEq)]
pub struct Info {
    pub count: i32,
    pub pages: i32,
    pub next: Option<String>,
    pub prev: Option<String>,
}

pub enum Resources {
    Character,
    Episode,
    Location,
}

pub struct Resource {
    resources: Resources,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone, Default)]
pub struct Object {
    pub name: String,
    pub url: String,
}

impl Resource {
    pub fn new(r: Resources) -> Self {
        Resource { resources: r }
    }

    fn base_url(&self) -> String {
        #[cfg(not(test))]
        let url = "https://rickandmortyapi.com";

        #[cfg(test)]
        let url = &mockito::server_url();

        match self.resources {
            Resources::Character => url.to_owned() + "/api/character",
            Resources::Episode => url.to_owned() + "/api/location",
            Resources::Location => url.to_owned() + "/api/episode",
        }
    }

    //TODO: make it multithreaded
    pub async fn get_multiple<T>(&self, pages: Vec<i32>) -> Result<Vec<T>, Error>
    where
        T: DeserializeOwned,
    {
        let mut page_query = String::from("");
        for page in pages.iter() {
            page_query = page_query + &page.to_string() + ",";
        }
        let mut url = self.base_url() + "/" + &page_query;
        url = url[0..url.len() - 1].parse().unwrap();
        let resp = get_url::<Vec<T>>(&url).await?;
        Ok(resp)
    }
}
