use clap::Parser;
use reqwest::header;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

const GITLAB_URL: &str = "https://gitlab.com/api/v4/projects";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// GitLab project name
    #[arg(short, long)]
    pub project_name: String,

    /// GitLab private token
    #[arg(short = 't', long)]
    pub private_token: String,
}

pub trait Request {
    fn get(&self, url: String) -> Result<String, Box<dyn std::error::Error>>;
    fn post(
        &self,
        url: String,
        body: &HashMap<String, String>,
    ) -> Result<String, Box<dyn std::error::Error>>;
}

struct GitLabRequest {
    token: String,
}

impl Request for GitLabRequest {
    fn get(&self, url: String) -> Result<String, Box<dyn std::error::Error>> {
        let mut headers = header::HeaderMap::new();
        let token = header::HeaderValue::try_from(self.token.clone()).unwrap();
        headers.insert("PRIVATE-TOKEN", token);
        let client = reqwest::blocking::Client::builder()
            .default_headers(headers)
            .build()?;
        let result = client.get(url).send()?.text()?;
        Ok(result)
    }

    fn post(
        &self,
        url: String,
        body: &HashMap<String, String>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();
        let result = client.post(url).json(body).send()?.text()?;
        Ok(result)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Project {
    id: u32,
    name: String,
    path: String,
    path_with_namespace: String,
}

fn main() {
    let args = Args::parse();
    let request = GitLabRequest {
        token: args.private_token,
    };
    let projects = get_project(&request, &args.project_name);
    println!("{:#?}", projects);
}

fn get_project(
    request: &impl Request,
    project_name: &String,
) -> Result<Vec<Project>, Box<dyn std::error::Error>> {
    let url = format!(
        "{0}/?search={1}&visibility=private&simple=true",
        GITLAB_URL, project_name
    );
    let projects: Vec<Project> = serde_json::from_str(&request.get(url)?)?;
    Ok(projects)
}
