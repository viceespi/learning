altura = input ("Qual a altura da figura?: ")
altura = int(altura)
largura = input ("Qual a largura da figura?: ")
largura = int(largura)
resultado = altura * largura
if largura == altura:
    print ("É um quadrado!")
    print ("A área é: ", resultado)
else: 
    print ("É um retângulo!")
    print ("A área é: ", resultado)
