def move_zeros(lst: list):
    for i in range(0, len(lst)):
        if lst[i] == 0:
            lst.remove(lst[i])
            lst.append(0)
    
    return lst