numero = [73, 21, 94, 42, 10, 87, 58, 35, 66, 5]
b = 1
while b < len(numero):
    if numero[b-1] < numero[b]:
        vt = numero[b-1]
        numero[b-1] = numero[b]
        numero[b] = vt
        b = 1
    else:
        b = b + 1
print(numero)