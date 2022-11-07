use crate::api::api::*;
use serde::{Deserialize, Serialize};

pub mod episode {
    use super::*;

    #[derive(Deserialize, Debug, PartialEq, Clone, Serialize, Default)]
    pub struct Episode {
        pub id: i32,
        pub name: String,
        pub air_date: Option<String>,
        pub episode: Option<String>,
        pub characters: Option<Vec<String>>,
        pub url: Option<String>,
        pub created: Option<String>,
    }

    impl Episode {}

    pub async fn get_multiple(pages: Vec<i32>) -> Result<Vec<Episode>, Error> {
        Resource::new(Resources::Episode)
            .get_multiple::<Episode>(pages)
            .await
    }
}
