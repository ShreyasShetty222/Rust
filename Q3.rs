//3. Return the shortest word in a string in Python


def shortest_word(s: str) -> str:
    words = s.split()
    return min(words, key=len)

# Example usage
print(shortest_word("find the shortest word in this sentence"))  # "in"
