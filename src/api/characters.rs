use crate::api::api::*;
use serde::{Deserialize, Serialize};

pub mod character {
    use super::*;

    #[derive(Deserialize, Debug, PartialEq, Clone, Serialize, Default)]
    pub struct Character {
        pub id: i32,
        pub name: String,
        pub status: String,
        pub species: String,
        #[serde(rename = "type")]
        pub character_type: Option<String>,
        pub gender: String,
        pub origin: Object,
        pub location: Object,
        pub image: String,
        pub episode: Vec<String>,
        pub url: Option<String>,
        pub created: Option<String>,
    }

    impl Character {}

    pub async fn get_multiple(pages: Vec<i32>) -> Result<Vec<Character>, Error> {
        Resource::new(Resources::Character)
            .get_multiple::<Character>(pages)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::mock;

    #[tokio::test]
    async fn it_gets_a_character() {
        let data = "[{ \"id\": 1, \"name\": \"John Doe\", \"status\": \"Alive\", \"species\": \"Human\", \"type\": \"\", \"gender\": \"Male\", \"origin\": { \"name\": \"\", \"url\": \"\" }, \"location\": { \"name\": \"\", \"url\": \"\" }, \"image\": \"mock.jpeg\", \"episode\": [], \"url\": \"mock.mock\", \"created\": \"mock\" }]";

        let _m = mock("GET", "/api/character/1,2,3,4")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(data)
            .create();

        let mut cha = character::Character::default();
        cha.id = 1;
        cha.name = "John Doe".to_string();
        cha.status = "Alive".to_string();
        cha.character_type = Option::from("".to_string());
        cha.gender = "Male".to_string();
        cha.image = "mock.jpeg".to_string();
        cha.url = Option::from("mock.mock".to_string());
        cha.created = Option::from("mock".to_string());
        cha.species = "Human".to_string();
        let mut expected = Vec::new();
        expected.push(cha);

        let req = character::get_multiple(vec![1, 2, 3, 4]).await;
        match req {
            Ok(c) => {
                println!("{:?}", c);
                assert_eq!(c, expected)
            }
            Err(e) => {
                println!("request error: {:?}", e);
                panic!("request failed");
            }
        }
    }
}
