use std::path::PathBuf;

use crate::mecab::generate_wordcloud_input;
use anyhow::Result;
use derive_new::new;
use fxhash::FxHashMap;
use serde::{Deserialize, Serialize};

type Year = u32;

#[derive(Debug, Clone, Default, new, Serialize, Deserialize)]
pub struct Section {
    pub id: String,
    pub title: String,
    pub url: String,
    pub time: String,
}

#[derive(Debug, Clone, Default, new, Serialize, Deserialize)]
pub struct Author {
    pub name: String,
    pub affiliation: String,
}

#[derive(Debug, Clone, Default, new, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub title: String,
    pub url: String,
    pub time: String,
    #[serde(rename = "abstract")]
    pub abstract_text: String,
    pub authors: Vec<Author>,
    pub keywords: Vec<String>,
    pub section: Section,
}

impl Session {
    pub fn title_with_id(&self) -> String {
        format!("[{}] {}", self.title, self.id)
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, new)]
pub struct StatsItem {
    pub name: String,
    pub value: f64,
    pub description: String,
    pub year: u32,
    pub titles: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, new)]
pub struct Stats {
    pub items: FxHashMap<Year, Vec<StatsItem>>,
}

impl Stats {
    fn filter_sessions_by_keyword(
        &self,
        sessions: &[Session],
        keywords: Vec<String>,
    ) -> Vec<Session> {
        sessions
            .iter()
            .filter(|session| {
                session.keywords.iter().any(|k| keywords.contains(k))
                    || keywords.iter().any(|k| session.title.contains(k))
                    || keywords.iter().any(|k| session.abstract_text.contains(k))
            })
            .cloned()
            .collect()
    }

    fn keyword_analysis(
        &mut self,
        sessions: &[Session],
        keywords: Vec<String>,
        title: &str,
        description: &str,
        year: Year,
    ) -> Result<()> {
        let filtered_sessions = self.filter_sessions_by_keyword(&sessions, keywords.clone());
        self.items
            .get_mut(&year)
            .unwrap_or(&mut Vec::new())
            .push(StatsItem::new(
                title.into(),
                filtered_sessions.len() as f64,
                description.into(),
                year,
                filtered_sessions
                    .iter()
                    .map(|s| s.title_with_id())
                    .collect::<Vec<String>>(),
            ));
        Ok(())
    }

    pub fn analyze(
        &mut self,
        year: Year,
        sessions: Vec<Session>,
        output_dir: PathBuf,
    ) -> Result<()> {
        self.items.insert(year, Vec::new());
        // Filter sessions to include only those that are relevant
        let sessions: Vec<Session> = sessions
            .iter()
            .filter(|s| {
                s.id.contains("GS") // General Sessions
                    || s.id.contains("-OS-")    // Organized Sessions
                    || s.id.contains("-Win-")   // Poster Sessions
                    || s.id.contains("-PS-")    // Keynote Sessions
                    || s.id.contains("-SS-")    // Student Sessions
                    || s.id.contains("-KS-") // Kikaku Sessions
            })
            .cloned()
            .collect();

        // ====== Total number of sessions ======
        {
            self.items
                .get_mut(&year)
                .unwrap_or(&mut Vec::new())
                .push(StatsItem::new(
                    "Total Sessions".to_string(),
                    sessions.len() as f64,
                    "Total number of sessions".into(),
                    year,
                    Vec::new(),
                ));
        }

        // ====== Generate wordcloud text input ======
        {
            // Tokenize the abstracts and titles for word cloud generation
            let abstracts: Vec<String> = sessions.iter().map(|s| s.abstract_text.clone()).collect();
            let titles: Vec<String> = sessions.iter().map(|s| s.title.clone()).collect();
            let all_text: String = format!("{} {}", abstracts.join(" "), titles.join(" "));
            let wordcloud_input = generate_wordcloud_input(&all_text);
            let output_path = output_dir.join(format!("wordcloud_input_text_{}.txt", year));
            std::fs::write(output_path.to_str().unwrap(), wordcloud_input)?;
        }

        // ====== Keyword analysis: AI, 人工知能, Artificial Intelligence ======
        {
            self.keyword_analysis(
                &sessions,
                vec![
                    "AI".into(),
                    "人工知能".into(),
                    "Artificial Intelligence".into(),
                ],
                "AI Sessions",
                "Number of sessions that contains the keywords 'AI', '人工知能', or 'Artificial Intelligence'",
                year,
            )?;
        }

        // Keyword analysis: AIエージェント, AI Agent
        let ai_agent_keywords = vec!["AIエージェント".into(), "AI Agent".into()];
        self.keyword_analysis(
            &sessions,
            ai_agent_keywords,
            "AI Agent Sessions",
            "Number of sessions that contains the keywords 'AIエージェント' or 'AI Agent'",
            year,
        )?;

        // Keyword analysis: 深層学習, ディープラーニング, Deep Learning
        {
            self.keyword_analysis(
                &sessions,
                vec![
                    "深層学習".into(),
                    "ディープラーニング".into(),
                    "Deep Learning".into(),
                ],
                "Deep Learning Sessions",
                "Number of sessions that contains the keywords '深層学習', 'ディープラーニング', or 'Deep Learning'",
                year,
            )?;
        }

        // Save the stats to a JSON file
        let stats_file = output_dir.join(format!("jsai_{}_stats.json", year));
        std::fs::write(stats_file.clone(), serde_json::to_string_pretty(&self)?)?;
        println!(
            "Analysis completed. Stats saved to {}",
            stats_file.display()
        );

        Ok(())
    }
}
