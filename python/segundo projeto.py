nome = "Anguzito"
valor = input (nome + ", quantos reais você recebeu?: ")
valor = int(valor)
if 10 <= valor <= 20:
    print (nome + ", aí sim hein, pode comprar bolinhas!!")
if valor < 10:
    print (nome + ", a pobreza é foda mesmo...")
if valor > 20:
    print ("Caralho, " + nome + ", tá nadando no dinheiro hein????")