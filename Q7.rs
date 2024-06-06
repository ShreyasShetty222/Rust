//7. Return the kth smallest element in an array in Python

def kth_smallest(arr: list, k: int) -> int:
    arr.sort()
    return arr[k-1]

# Example usage
print(kth_smallest([3, 2, 1, 5, 6, 4], 2))  # 2
print(kth_smallest([3, 2, 3, 1, 2, 4, 5, 5, 6], 4))  # 3
