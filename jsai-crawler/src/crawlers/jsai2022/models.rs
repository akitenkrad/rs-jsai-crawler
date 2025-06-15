use crate::stats::models::{Author, Section, Session};
use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, new, Serialize, Deserialize)]
pub struct Section2022 {
    pub id: String,
    pub title: String,
    pub url: String,
    pub time: String,
}

impl From<Section2022> for Section {
    fn from(section: Section2022) -> Self {
        let Section2022 {
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
pub struct Author2022 {
    pub name: String,
    pub affiliation: String,
}

impl From<Author2022> for Author {
    fn from(author: Author2022) -> Self {
        let Author2022 { name, affiliation } = author;
        Author { name, affiliation }
    }
}

#[derive(Debug, Clone, Default, new, Serialize, Deserialize)]
pub struct Session2022 {
    pub id: String,
    pub title: String,
    pub url: String,
    pub time: String,
    #[serde(rename = "abstract")]
    pub abstract_text: String,
    pub authors: Vec<Author2022>,
    pub keywords: Vec<String>,
    pub section: Section2022,
}

impl From<Session2022> for Session {
    fn from(session: Session2022) -> Self {
        let Session2022 {
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

impl Session2022 {
    pub fn title_with_id(&self) -> String {
        format!("[{}] {}", self.id, self.title)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonData2022 {
    pub sections: Vec<Section2022>,
    pub sessions: Vec<Session2022>,
}
