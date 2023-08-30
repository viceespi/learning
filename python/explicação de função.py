
def quantidadenotas (total,nota):
    notas = total // nota
    print (notas,"nota(s) de R${},00".format(nota))
    novototal = total - (nota * notas)
    return novototal

valor = int(input())
notas = [100,50,20,10,5,2,1]
for elemento in notas:
    valor = quantidadenotas(valor, elemento)