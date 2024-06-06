//1. Check if a string is a palindrome in Python

def is_palindrome(s: str) -> bool:
    return s == s[::-1]

# Example usage
print(is_palindrome("radar"))  # True
print(is_palindrome("hello"))  # False
