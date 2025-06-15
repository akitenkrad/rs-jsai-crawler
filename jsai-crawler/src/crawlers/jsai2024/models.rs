use crate::stats::models::{Author, Section, Session};
use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, new, Serialize, Deserialize)]
pub struct Section2024 {
    pub id: String,
    pub title: String,
    pub url: String,
    pub time: String,
}

impl From<Section2024> for Section {
    fn from(section: Section2024) -> Self {
        let Section2024 {
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
pub struct Author2024 {
    pub name: String,
    pub affiliation: String,
}

impl From<Author2024> for Author {
    fn from(author: Author2024) -> Self {
        let Author2024 { name, affiliation } = author;
        Author { name, affiliation }
    }
}

#[derive(Debug, Clone, Default, new, Serialize, Deserialize)]
pub struct Session2024 {
    pub id: String,
    pub title: String,
    pub url: String,
    pub time: String,
    #[serde(rename = "abstract")]
    pub abstract_text: String,
    pub authors: Vec<Author2024>,
    pub keywords: Vec<String>,
    pub section: Section2024,
}

impl From<Session2024> for Session {
    fn from(session: Session2024) -> Self {
        let Session2024 {
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

impl Session2024 {
    pub fn title_with_id(&self) -> String {
        format!("[{}] {}", self.id, self.title)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonData2024 {
    pub sections: Vec<Section2024>,
    pub sessions: Vec<Session2024>,
}
