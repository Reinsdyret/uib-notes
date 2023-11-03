
def countInversions(arr):
    if len(arr) == 1: return 0, arr

    countA, sortedA = countInversions(arr[:len(arr)//2])
    countB, sortedB = countInversions(arr[len(arr)//2:])

    finalSorted = []
    i = 0
    j = 0
    inversions = countA + countB

    while i < len(sortedA) and j < len(sortedB):
        if sortedA[i] <= sortedB[j]:
            finalSorted.append(sortedA[i])
            i += 1
        else:
            finalSorted.append(sortedB[j])
            j += 1
            inversions += 1

    finalSorted.extend(sortedA[i:])
    finalSorted.extend(sortedB[j:])

    return inversions, finalSorted

def count_inversions(A, B):
    # Create a mapping from elements in list A to their indices
    index_mapping = {val: i for i, val in enumerate(A)}
    # Relabel items in list B according to their indices in list A
    relabeled_B = [index_mapping[val] for val in B]

    return countInversions(relabeled_B)

# Example usage:
A = [4, 3, 2, 1, 0]
B = [0, 1, 2, 3, 4]
print(count_inversions(A, B))


