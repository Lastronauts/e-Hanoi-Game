use actix_web::http::header;
use anyhow::Result;
use graphql_client::{GraphQLQuery, Response};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::BufReader};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./schema.graphql",
    query_path = "./queries.graphql",
    response_derives = "Debug"
)]
struct CreateScore;

pub async fn create_score(graphql_endpoint: &str) -> Result<()> {
    let new_score = NewScore::reader()?;
    let input = create_score::Variables {
        new_score: new_score.clone().into(),
    };

    let client = Client::new();
    let req_body = CreateScore::build_query(input);

    let res = client
        .post(graphql_endpoint)
        .header(header::AUTHORIZATION, new_score.token)
        .json(&req_body)
        .send()
        .await?;
        println!("{:#?}", res);
    let res_body: Response<create_score::ResponseData> = res.json().await?;
    println!("{:#?}", res_body);

    Ok(())
}

#[derive(Clone, Serialize, Deserialize)]
struct NewScore {
    token: String,
    clear_time: i64,
}

impl NewScore {
    pub fn reader() -> Result<NewScore> {
        let file = File::open("tmp/query.json")?;
        let reader = BufReader::new(file);

        let new_score = serde_json::from_reader::<_, NewScore>(reader)?;

        Ok(new_score)
    }
}

impl From<NewScore> for create_score::NewScore {
    fn from(new_score: NewScore) -> Self {
        Self {
            clearTime: new_score.clear_time,
        }
    }
}
