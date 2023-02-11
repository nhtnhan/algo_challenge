# https://www.algoexpert.io/questions/validate-subsequence

def isValidSubsequence(array, sequence):
    i = 0
    j = 0

    print(array,len(array))
    print(sequence,len(sequence))
    
    while (i != len(sequence) and j != len(array)):
        print(i,j)
        
        if sequence[i] != array[j]:
            j+=1
        else:
            i+=1
            j+=1

    if i == len(sequence):
        return True
    return False
