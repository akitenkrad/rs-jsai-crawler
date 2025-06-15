use core::str;

use crate::stats::models::Author;
use anyhow::Result;
use dotenvy::dotenv;
use openai_tools::{Message, OpenAI, ResponseFormat, json_schema::JsonSchema};
use serde::{Deserialize, Serialize};

type AuthorName = String;
type AuthorAffiliation = String;

#[derive(Debug, Default, Deserialize, Serialize)]
struct ResAuthors {
    authors: Vec<Author>,
}

pub fn parse_authors(session_text: &str) -> Result<Vec<(AuthorName, AuthorAffiliation)>> {
    dotenv().ok();
    let model_id = std::env::var("OPENAI_MODEL_ID").expect("OPENAI_MODEL_ID must be set");
    let mut openai = OpenAI::new();
    let messages = vec![
        Message::new(
            String::from("system"),
            String::from("あなたは自然言語に関する世界トップレベルの研究者です．"),
        ),
        Message::new(
            String::from("user"),
            format!(
                r#"
# Instruction
次のHTMLは論文の著者と所属を記述したものです．このHTMLを解析して，以下の情報を含むJSON形式のテキストを出力してください．
著者の名前と所属は番号で対応しています．

- 著者の名前
- 著者の所属

# JSON形式の出力例

```json
[
    {{
        "name": "著者名1",
        "affiliation": "所属1"
    }},
    {{
        "name": "著者名2",
        "affiliation": "所属2"
    }}
]
```

# HTML

```
{html}
```
"#,
                html = session_text
            ),
        ),
    ];

    let mut json_schema = JsonSchema::new(String::from("session"));
    json_schema.add_array(
        "authors".into(),
        vec![
            (
                "name".into(),
                r#"講演の発表者の名前を記載してください．"#.into(),
            ),
            (
                "affiliation".into(),
                r#"講演の発表者の所属を記載してください．"#.into(),
            ),
        ],
    );
    let response_format = ResponseFormat::new("json_schema".to_string(), json_schema);
    openai
        .model_id(model_id)
        .messages(messages)
        .temperature(1.0)
        .response_format(response_format);

    let response = openai.chat().unwrap();
    if response.choices.is_empty() {
        return Err(anyhow::anyhow!("No choices returned from OpenAI API"));
    }
    let authors = match serde_json::from_str::<ResAuthors>(&response.choices[0].message.content) {
        Ok(authors) => Ok(authors.authors),
        Err(e) => Err(anyhow::anyhow!(
            "Failed to parse author: {}. Response: {}",
            e,
            response.choices[0].message.content
        )),
    }?;
    let parsed_authors: Vec<(AuthorName, AuthorAffiliation)> = authors
        .into_iter()
        .map(|author| (author.name, author.affiliation))
        .collect();
    Ok(parsed_authors)
}
