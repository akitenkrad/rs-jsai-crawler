use crate::crawlers::jsai2025::models::{Author2025, JsonData2025, Section2025, Session2025};
use crate::shared::utils::create_progress_bar;
use crate::stats::ai::parse_authors;
use crate::stats::models::Session;
use anyhow::Result;
use regex::Regex;
use std::error::Error;
use thirtyfour::prelude::*;
use url::Url;

async fn collect_sections(
    driver: &WebDriver,
) -> Result<Vec<Section2025>, Box<dyn Error + Send + Sync>> {
    let mut result_sections = Vec::new();

    let title_regex = Regex::new(r"^\[(?P<id>.+?)\](?P<title>.+)$")?;

    let sections = driver.find_all(By::Css("section article")).await?;
    for section in sections {
        let title = section.find(By::Css("div.title")).await?.text().await?;
        let id = title_regex
            .captures(&title)
            .and_then(|caps| caps.name("id").map(|m| m.as_str().to_string()))
            .unwrap_or_default();
        let title = title_regex
            .captures(&title)
            .and_then(|caps| caps.name("title").map(|m| m.as_str().to_string()))
            .unwrap_or_default();
        let url = section
            .find(By::Css("div.title a"))
            .await?
            .attr("href")
            .await?
            .unwrap_or_default();
        let url = Url::parse("https://confit.atlas.jp")
            .expect("Failed to parse base URL")
            .join(&url)
            .expect("Failed to join URL")
            .to_string();
        let time = section
            .find(By::Css("div.content p.date > span"))
            .await?
            .text()
            .await?;

        result_sections.push(Section2025::new(id, title, url, time));
    }
    Ok(result_sections)
}

async fn extracx_session_url(
    driver: &WebDriver,
    section: &Section2025,
) -> Result<Vec<String>, Box<dyn Error + Send + Sync>> {
    driver.goto(&section.url).await?;
    let mut session_urls = Vec::new();
    let session_elements = driver
        .find_all(By::Css("section article div.sbjtitle h1 a"))
        .await?;
    for element in session_elements {
        if let Some(href) = element.attr("href").await? {
            let full_url = Url::parse("https://confit.atlas.jp")
                .expect("Failed to parse base URL")
                .join(&href)
                .expect("Failed to join URL")
                .to_string();
            session_urls.push(full_url);
        }
    }
    Ok(session_urls)
}

async fn parse_session(
    driver: &WebDriver,
    url: &str,
    section: &Section2025,
) -> Result<Session2025> {
    driver.goto(url).await?;
    let article = driver
        .find(By::Css("section article.sbject-single"))
        .await?;
    let h1 = article
        .find(By::Css("div.title h1"))
        .await?
        .text()
        .await?
        .trim()
        .to_string();
    let id_regex = Regex::new(r"^\[(?P<id>.+?)\]\s*(?P<title>.+?)$")?;
    let id = id_regex
        .captures(&h1)
        .and_then(|caps| caps.name("id").map(|m| m.as_str().to_string()))
        .unwrap_or_default();
    let title = id_regex
        .captures(&h1)
        .and_then(|caps| caps.name("title").map(|m| m.as_str().to_string()))
        .unwrap_or_default();
    let time = article
        .find(By::Css("div.clear p.date"))
        .await?
        .text()
        .await?
        .trim()
        .to_string();
    let time = format!("{} ({})", section.time, time);
    let abstract_text = match article.find(By::Css("div.content div.outline")).await {
        Ok(abstract_element) => abstract_element.text().await?.trim().to_string(),
        Err(_) => String::new(),
    };
    let authors_html = article
        .find(By::Css("div.content p.personals.author"))
        .await?
        .text()
        .await?
        .trim()
        .to_string();
    let authors = parse_authors(&authors_html)?;
    let authors: Vec<Author2025> = authors
        .into_iter()
        .map(|(name, affiliation)| Author2025::new(name, affiliation))
        .collect();
    let keywords: Vec<String> = match article.find(By::Css("div.content p.keyword")).await {
        Ok(keyword_element) => keyword_element
            .text()
            .await?
            .replace("キーワード：", "")
            .trim()
            .split('、')
            .map(|s| s.trim().to_string())
            .collect(),
        Err(_) => Vec::new(),
    };

    Ok(Session2025::new(
        id,
        title,
        url.to_string(),
        time,
        abstract_text,
        authors,
        keywords,
        section.clone(),
    ))
}

pub async fn crawl_jsai2025() -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut caps = DesiredCapabilities::chrome();
    caps.add_arg("--headless=new")?;

    let driver = WebDriver::new("http://localhost:51876", caps).await?;
    println!("WebDriver started successfully");

    driver
        .goto("https://confit.atlas.jp/guide/event/jsai2025/date")
        .await?;

    // collect sections for each day
    let mut sections: Vec<Section2025> = Vec::new();
    let days = vec!["20250527", "20250528", "20250529", "20250530"];
    let pb = create_progress_bar(days.len(), Some("Collecting sections".to_string()));
    for day in days {
        driver
            .goto(format!(
                "https://confit.atlas.jp/guide/event/jsai2025/sessions/date/{}?page=1",
                day
            ))
            .await?;
        loop {
            let ss = collect_sections(&driver).await?;
            sections.extend(ss.clone());

            match driver.find(By::Css("#pageNavHead li:last-child a")).await {
                Ok(next_button) => {
                    let next_text = next_button.text().await?;
                    if next_text.contains("次へ") {
                        // click the next button
                        driver
                            .action_chain()
                            .click_element(&next_button)
                            .perform()
                            .await?;
                        std::thread::sleep(std::time::Duration::from_secs(1)); // wait for the page to load
                    } else {
                        // no more pages
                        break;
                    }
                }
                Err(_) => {
                    // no next button found, break the loop
                    break;
                }
            }
        }
        pb.inc(1);
    }
    pb.finish_with_message("Section collection completed");

    // collect sections
    let mut sessions: Vec<Session2025> = Vec::new();
    let pb = create_progress_bar(sections.len(), Some("Collecting sessions".to_string()));
    for section in &sections {
        let session_urls = match extracx_session_url(&driver, section).await {
            Ok(urls) => urls,
            Err(e) => {
                pb.println(format!(
                    "Error extracting session URLs ({}): {}",
                    section.url, e
                ));
                pb.inc(1);
                continue;
            }
        };
        for session_url in session_urls {
            let session = match parse_session(&driver, &session_url, section).await {
                Ok(session) => session,
                Err(e) => {
                    pb.println(format!("Error parsing session ({}): {}", session_url, e));
                    continue;
                }
            };
            sessions.push(session);
            pb.set_message(format!("Collected {} sessions", sessions.len()));
        }

        pb.inc(1);
        std::thread::sleep(std::time::Duration::from_secs(1)); // wait for the page to load
    }
    pb.finish_with_message("Session collection completed");

    // save sections and sessions into a JSON file
    let json = serde_json::json!({
        "sections": sections,
        "sessions": sessions
    });
    std::fs::write("jsai2025.json", json.to_string())?;

    driver.quit().await?;
    Ok(())
}

pub fn load_sessions_from_json(
    file_path: &str,
) -> Result<Vec<Session>, Box<dyn Error + Send + Sync>> {
    let data = std::fs::read_to_string(file_path)?;
    let json_data: JsonData2025 = serde_json::from_str(&data)?;
    Ok(json_data.sessions.into_iter().map(Session::from).collect())
}
