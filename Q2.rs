//2. Find the index of the first occurrence of a given number in a sorted array in Python



def first_occurrence(arr: list, target: int) -> int:
    left, right = 0, len(arr) - 1
    while left <= right:
        mid = (left + right) // 2
        if arr[mid] == target:
            if mid == 0 or arr[mid - 1] < target:
                return mid
            right = mid - 1
        elif arr[mid] < target:
            left = mid + 1
        else:
            right = mid - 1
    return -1

# Example usage
print(first_occurrence([1, 2, 2, 2, 3, 4], 2))  # 1
