# rs-jsai-crawler

## Crawler

-> jsai-crawler

```txt
Usage: jsai-crawler <COMMAND>

Commands:
  crawl-jsai2021  
  crawl-jsai2022  
  crawl-jsai2023  
  crawl-jsai2024  
  crawl-jsai2025  
  analyze         
  help            Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Generate wordcloud

-> gen_wordcloud

```txt
NAME
    main.py - Generate a word cloud from the text file at text_path and save it to output_path.

SYNOPSIS
    uv run main.py TEXT_PATH OUTPUT_PATH <flags>

DESCRIPTION
    Generate a word cloud from the text file at text_path and save it to output_path.

POSITIONAL ARGUMENTS
    TEXT_PATH
        Type: str
        Path to the input text file.
    OUTPUT_PATH
        Type: str
        Path where the word cloud image will be saved.

FLAGS
    -f, --font=FONT
        Type: str
        Default: 'HackGenConsoleNF-Bold.ttf'
    -s, --stopwords=STOPWORDS
        Type: str
        Default: 'stopwords.csv'

NOTES
    You can also use flags syntax for POSITIONAL ARGUMENTS
```
