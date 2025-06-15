pub mod crawlers;
pub mod mecab;
pub mod shared;
pub mod stats;

use crate::crawlers::{
    jsai2021::kernel::crawl_jsai2021, jsai2022::kernel::crawl_jsai2022,
    jsai2023::kernel::crawl_jsai2023, jsai2024::kernel::crawl_jsai2024,
    jsai2025::kernel::crawl_jsai2025,
};
use crate::stats::models::Stats;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    subcommand: SubCommands,
}

#[derive(Subcommand, Debug)]
enum SubCommands {
    #[command(name = "crawl-jsai2021")]
    CrawlJsai2021,
    #[command(name = "crawl-jsai2022")]
    CrawlJsai2022,
    #[command(name = "crawl-jsai2023")]
    CrawlJsai2023,
    #[command(name = "crawl-jsai2024")]
    CrawlJsai2024,
    #[command(name = "crawl-jsai2025")]
    CrawlJsai2025,
    #[command(name = "analyze")]
    Analyze(AnalyzeArgs),
}

#[derive(Parser, Debug)]
struct AnalyzeArgs {
    /// Path to the JSAI 2025 data file
    #[arg(short, long)]
    data: String,
    #[arg(short, long)]
    year: u32,
    #[arg(short, long, default_value = "output")]
    output_dir: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    match args.subcommand {
        SubCommands::CrawlJsai2021 => {
            println!("Crawling JSAI 2021 data...");
            if let Err(e) = crawl_jsai2021().await {
                eprintln!("Error crawling JSAI 2021: {}", e);
            }
        }
        SubCommands::CrawlJsai2022 => {
            println!("Crawling JSAI 2022 data...");
            if let Err(e) = crawl_jsai2022().await {
                eprintln!("Error crawling JSAI 2022: {}", e);
            }
        }
        SubCommands::CrawlJsai2023 => {
            println!("Crawling JSAI 2023 data...");
            if let Err(e) = crawl_jsai2023().await {
                eprintln!("Error crawling JSAI 2023: {}", e);
            }
        }
        SubCommands::CrawlJsai2024 => {
            println!("Crawling JSAI 2024 data...");
            if let Err(e) = crawl_jsai2024().await {
                eprintln!("Error crawling JSAI 2024: {}", e);
            }
        }
        SubCommands::CrawlJsai2025 => {
            if let Err(e) = crawl_jsai2025().await {
                eprintln!("Error crawling JSAI 2025: {}", e);
            }
        }
        SubCommands::Analyze(args) => {
            println!("Analyzing JSAI {} data from file: {}", args.year, args.data);

            let mut sessions = Vec::new();
            if args.year == 2021 {
                sessions = crawlers::jsai2021::kernel::load_sessions_from_json(&args.data)
                    .expect("Failed to load JSAI 2021 sessions");
            } else if args.year == 2022 {
                sessions = crawlers::jsai2022::kernel::load_sessions_from_json(&args.data)
                    .expect("Failed to load JSAI 2022 sessions");
            } else if args.year == 2023 {
                sessions = crawlers::jsai2023::kernel::load_sessions_from_json(&args.data)
                    .expect("Failed to load JSAI 2023 sessions");
            } else if args.year == 2024 {
                sessions = crawlers::jsai2024::kernel::load_sessions_from_json(&args.data)
                    .expect("Failed to load JSAI 2024 sessions");
            } else if args.year == 2025 {
                sessions = crawlers::jsai2025::kernel::load_sessions_from_json(&args.data)
                    .expect("Failed to load JSAI 2025 sessions");
            }

            let mut stats = Stats::default();
            if let Err(e) =
                stats.analyze(args.year, sessions, PathBuf::from(args.output_dir.unwrap()))
            {
                eprintln!("Error analyzing data: {}", e);
            } else {
                println!("Analysis completed successfully.");
            }
        }
    }
}
