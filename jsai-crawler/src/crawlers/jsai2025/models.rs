use crate::stats::models::{Author, Section, Session};
use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, new, Serialize, Deserialize)]
pub struct Section2025 {
    pub id: String,
    pub title: String,
    pub url: String,
    pub time: String,
}

impl From<Section2025> for Section {
    fn from(section: Section2025) -> Self {
        let Section2025 {
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
pub struct Author2025 {
    pub name: String,
    pub affiliation: String,
}

impl From<Author2025> for Author {
    fn from(author: Author2025) -> Self {
        let Author2025 { name, affiliation } = author;
        Author { name, affiliation }
    }
}

#[derive(Debug, Clone, Default, new, Serialize, Deserialize)]
pub struct Session2025 {
    pub id: String,
    pub title: String,
    pub url: String,
    pub time: String,
    #[serde(rename = "abstract")]
    pub abstract_text: String,
    pub authors: Vec<Author2025>,
    pub keywords: Vec<String>,
    pub section: Section2025,
}

impl From<Session2025> for Session {
    fn from(session: Session2025) -> Self {
        let Session2025 {
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

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonData2025 {
    pub sections: Vec<Section2025>,
    pub sessions: Vec<Session2025>,
}
