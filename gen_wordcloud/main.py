from fire import Fire


def generate_wordcloud(text_path: str, output_path: str):
    """
    Generate a word cloud from the text file at text_path and save it to output_path.
    
    :param text_path: Path to the input text file.
    :param output_path: Path where the word cloud image will be saved.
    """
    import matplotlib.pyplot as plt
    from wordcloud import WordCloud

    with open("stopwords.en.csv", 'r', encoding='utf-8') as file:
        stopwords_en = file.read().splitlines()
    with open("stopwords.ja.csv", 'r', encoding='utf-8') as file:
        stopwords_ja = file.read().splitlines()
    stopwords = stopwords_en + stopwords_ja

    with open(text_path, 'r', encoding='utf-8') as file:
        text = file.read()

    word_cloud = WordCloud(font_path="HackGenConsoleNF-Bold.ttf", width=1500, height=1500,
                            stopwords=set(stopwords),min_font_size=5,
                            collocations=False, background_color='white',
                            max_words=400).generate(text)
    figure = plt.figure(figsize=(15,15))
    plt.imshow(word_cloud)
    plt.tick_params(labelbottom=False, labelleft=False)
    plt.xticks([])
    plt.yticks([])
    figure.savefig(output_path, dpi=300, bbox_inches='tight')

def main():
    Fire(generate_wordcloud)

if __name__ == "__main__":
    main()
