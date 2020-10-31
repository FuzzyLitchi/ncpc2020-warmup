import sys

letters = {
    'B': 1,
    'F': 1,
    'P': 1,
    'V': 1,
    'C': 2,
    'G': 2,
    'J': 2,
    'K': 2,
    'Q': 2,
    'S': 2,
    'X': 2,
    'Z': 2,
    'D': 3,
    'T': 3,
    'L': 4,
    'M': 5,
    'N': 5,
    'R': 6
}

data = []
for line in sys.stdin:
    data.append(line.split()[0])

for word in data:
    value = ""
    last = ""
    characters = list(word)
    for char in characters:
        if (char in letters):
            if (str(letters[char]) != last):
                value = value + str(letters[char])
                last = str(letters[char])
            continue
        last = ""
    print(value)
