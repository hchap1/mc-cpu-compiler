from math import log, floor

def int_to_bin(num):
    if num == 0:
        return "0"
    mut_num = num
    order = floor(log(num, 2))
    array = [0] * (order + 1)
    array[0] = 1
    mut_num -= 2**order
    idx = 0
    for n in range(order-1,-1,-1):
        if mut_num >= 2**n:
            mut_num -= 2**n
            array[idx] = 1
        idx += 1
    return "".join([str(x) for x in array])

print(int_to_bin(1223478))
