
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
            inversions += (len(sortedA) - i)

    finalSorted.extend(sortedA[i:])
    finalSorted.extend(sortedB[j:])

    return inversions, finalSorted

def count_inversions(A, B):
    """
    Given two lists A,B returns a tuple with (inversions, sorted) where inversions is the number of inversions needed for B to be equal to A, the sorted is the new list B after all inversions.

    input: 
        A (list): A list of comparable elements, this is the "ground truth" for list B
        B (list): A list of comparable elements, this is the list that we count inversions on until it becomes A.

    output:
        (inversions, sorted):
            inversions (int): The number of inversions needed to make B equal to A.
            sorted (list):    The new sorted list B after all inversions are done.

    Time complexity: 
        T(n) = 2T(n/2) + O(n), based on the masters theorem this is O(n log n)

    Preprocessing:
        O(n): relabeling the items in B based on A
    """
    # Create a mapping from elements in list A to their indices
    index_mapping = {val: i for i, val in enumerate(A)}
    # Relabel items in list B according to their indices in list A
    relabeled_B = [index_mapping[val] for val in B]

    return countInversions(relabeled_B)

# Example usage:
A = [4, 3, 2, 1, 0]
B = [0, 1, 2, 3, 4]
print(count_inversions(A, B))


