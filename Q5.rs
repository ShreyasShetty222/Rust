//5. Return the median of a sorted array in Python

def median(arr: list) -> float:
    n = len(arr)
    if n % 2 == 0:
        return (arr[n//2 - 1] + arr[n//2]) / 2
    return arr[n//2]

# Example usage
print(median([1, 2, 3, 4, 5]))  # 3
print(median([1, 2, 3, 4, 5, 6]))  # 3.5
