
'''
1 - criar a variável do input do dinheiro
2 - separar os elementos desse vetor em duas variáveis, dinheronota e dinheiromoeda
3 - elaborar um algorítmo que utilize a base
'''
dinheiro = input()
dinheironota = ""
dinheiromoeda = ""
achouponto = False
for element in dinheiro:
    if element == ".": achouponto = True
    else:
        if achouponto == True:
            dinheiromoeda = dinheiromoeda + element
        else: dinheironota = dinheironota + element
notas = [100, 50, 20, 10, 5, 2, 1]
moedas = [50, 25, 10, 5, 1]
dinheironota = int(dinheironota)
dinheiromoeda = int(dinheiromoeda)


def quantidade (total: int, currency: int):
    qntd = total // currency
    return qntd
def calcularnovovalor (total: int, qntd: int, currency: int):
    newmoney = total - (qntd * currency)
    return newmoney
print("NOTAS:")
for element in notas:
    qntdnotas = quantidade (dinheironota, element)
    dinheironota = calcularnovovalor(dinheironota, qntdnotas, element)
    if element == 1:
        print ("MOEDAS:")
        print(qntdnotas,"moeda(s) de R$ {:.2f}".format(element))
    else: 
        print(qntdnotas,"nota(s) de R$ {}.00".format(element))
for element in moedas:
    qntdmoedas = quantidade (dinheiromoeda, element)
    dinheiromoeda = calcularnovovalor (dinheiromoeda, qntdmoedas, element)
    moedareal = element / 100
    print(qntdmoedas,"moeda(s) de R$ {:.2f}".format(moedareal))




