//6. Find the longest common prefix in a set of strings in Python

def longest_common_prefix(strs: list) -> str:
    if not strs:
        return ""
    shortest = min(strs, key=len)
    for i, char in enumerate(shortest):
        for other in strs:
            if other[i] != char:
                return shortest[:i]
    return shortest

# Example usage
print(longest_common_prefix(["flower", "flow", "flight"]))  # "fl"
print(longest_common_prefix(["dog", "racecar", "car"]))  # ""
