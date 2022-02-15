from unidecode import unidecode

INPUT_FILE: str = 'Lexique383.tsv'
INPUT_SEPARATOR: str = '\t'
MIN_LENGTH: int = 3

OUTPUT_FILE: str = 'french_words.txt'
OUTPUT_SEPARATOR: str = '\n'

words: set = set()

with open(INPUT_FILE, 'r') as file:
    lines: list = [ str(line) for line in file.readlines() ]
    print(f'Reading file {INPUT_FILE} (Count: {len(lines)})')
    for index, line in enumerate(lines):
        word: str = unidecode(line.strip().split()[0])
        print(f'Word {index}: \'{word}\'', end=' ')
        if len(word) >= MIN_LENGTH and word.isalpha():
            words.add(word)
            print('added to set')
        else:
            print('ignored')

count: int = len(words)
OUTPUT_FILE_FINAL: str = OUTPUT_FILE.replace(".txt", f'_{count}.txt')
with open(OUTPUT_FILE_FINAL, 'w') as file:
    print(f'Writing file {OUTPUT_FILE_FINAL} (Count: {len(words)})')
    file.write(OUTPUT_SEPARATOR.join(sorted(words)))