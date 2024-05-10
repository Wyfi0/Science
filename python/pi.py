
divisor = 1
result = 0

for i in range(0, 1000000000):
    sub_result = 4/divisor

    if i % 2 == 0:
        result = result + sub_result
    else:
        result = result - sub_result

    divisor = divisor + 2

print(result)
