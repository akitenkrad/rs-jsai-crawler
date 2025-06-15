use crate::stats::models::{Author, Section, Session};
use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, new, Serialize, Deserialize)]
pub struct Section2021 {
    pub id: String,
    pub title: String,
    pub url: String,
    pub time: String,
}

impl From<Section2021> for Section {
    fn from(section: Section2021) -> Self {
        let Section2021 {
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
pub struct Author2021 {
    pub name: String,
    pub affiliation: String,
}

impl From<Author2021> for Author {
    fn from(author: Author2021) -> Self {
        let Author2021 { name, affiliation } = author;
        Author { name, affiliation }
    }
}

#[derive(Debug, Clone, Default, new, Serialize, Deserialize)]
pub struct Session2021 {
    pub id: String,
    pub title: String,
    pub url: String,
    pub time: String,
    #[serde(rename = "abstract")]
    pub abstract_text: String,
    pub authors: Vec<Author2021>,
    pub keywords: Vec<String>,
    pub section: Section2021,
}

impl From<Session2021> for Session {
    fn from(session: Session2021) -> Self {
        let Session2021 {
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

impl Session2021 {
    pub fn title_with_id(&self) -> String {
        format!("[{}] {}", self.id, self.title)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonData2021 {
    pub sections: Vec<Section2021>,
    pub sessions: Vec<Session2021>,
}
