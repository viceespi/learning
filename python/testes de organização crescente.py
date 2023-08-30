numero = [73, 21, 94, 42, 10, 87, 58, 35, 66, 5]
imin = 0
imax = ""
for i in range(0,(len(numero))):
    elemento = numero[i]
    if elemento == min(numero):
        imin = i
    
print(imin)

numero[0] = min(numero)
numero[(len(numero))-1] = max(numero) 
print(numero)

if numero[0] == 73:
    check = True


b = 1
while b != len(numero):
    a = 0
    b = 1
    va = numero[a]
    vb = numero[b]
    comparação = va < vb
    if comparação == False:
        vt = va
        va = vb
        numero[b] = vt
        a = 0
        b = 1
    else: 
        a = a + 1
        b = b + 1
    print(numero)