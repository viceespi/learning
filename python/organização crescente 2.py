num = [21,13,47,49,6,8,99,81,86,100]
casa_n_atual = 0
j = casa_n_atual
vetorcomp = range(casa_n_atual,len(num))
casa_n_menor = j
for element in range(casa_n_atual,(len(num) - 1)):
    j = element
    nummenor = num[element]
    while j < len(vetorcomp):
        numteste = num[j]
        if nummenor > numteste:
            number = numteste
            nummenor = number
            casa_n_menor = j
            j = j + 1
        else: j = j + 1
    vt = num[element]
    num[element] = nummenor
    num[casa_n_menor] = vt
print(num)
