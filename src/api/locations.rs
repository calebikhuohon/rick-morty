use crate::api::api::*;
use serde::{Deserialize, Serialize};

pub mod location {
    use super::*;

    #[derive(Deserialize, Debug, PartialEq, Clone, Serialize, Default)]
    pub struct Location {
        pub id: i32,
        pub name: String,
        #[serde(rename = "type")]
        pub character_type: Option<String>,
        pub dimension: Option<String>,
        pub residents: Option<Vec<String>>,
        pub url: Option<String>,
        pub created: Option<String>,
    }

    impl Location {}

    pub async fn get_multiple(pages: Vec<i32>) -> Result<Vec<Location>, Error> {
        Resource::new(Resources::Location)
            .get_multiple::<Location>(pages)
            .await
    }
}
