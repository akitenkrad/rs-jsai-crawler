use crate::stats::models::{Author, Section, Session};
use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, new, Serialize, Deserialize)]
pub struct Section2023 {
    pub id: String,
    pub title: String,
    pub url: String,
    pub time: String,
}

impl From<Section2023> for Section {
    fn from(section: Section2023) -> Self {
        let Section2023 {
            id,
            title,
            url,
            time,
        } = section;
        Section {
            id,
            title,
            url,
            time,
        }
    }
}

#[derive(Debug, Clone, Default, new, Serialize, Deserialize)]
pub struct Author2023 {
    pub name: String,
    pub affiliation: String,
}

impl From<Author2023> for Author {
    fn from(author: Author2023) -> Self {
        let Author2023 { name, affiliation } = author;
        Author { name, affiliation }
    }
}

#[derive(Debug, Clone, Default, new, Serialize, Deserialize)]
pub struct Session2023 {
    pub id: String,
    pub title: String,
    pub url: String,
    pub time: String,
    #[serde(rename = "abstract")]
    pub abstract_text: String,
    pub authors: Vec<Author2023>,
    pub keywords: Vec<String>,
    pub section: Section2023,
}

impl From<Session2023> for Session {
    fn from(session: Session2023) -> Self {
        let Session2023 {
            id,
            title,
            url,
            time,
            abstract_text,
            authors,
            keywords,
            section,
        } = session;
        Session {
            id,
            title,
            url,
            time,
            abstract_text,
            authors: authors.into_iter().map(Author::from).collect(),
            keywords,
            section: Section::from(section),
        }
    }
}

impl Session2023 {
    pub fn title_with_id(&self) -> String {
        format!("[{}] {}", self.id, self.title)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonData2023 {
    pub sections: Vec<Section2023>,
    pub sessions: Vec<Session2023>,
}
