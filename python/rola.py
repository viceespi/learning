''' O problema é: Receber uma quantidade N de caracteres, que podem ser letras (case 
sensitive), números ou caracteres especiais, e preciso identificar nesses caractéres, a
presença de no mínimo 4 letras: R O L A.
- O primeiro passo é identificar quantos caracteres existem no input do usuário, para então
criar um vetor do mesmo tamanho, sendo cada espaço 1 caracteres (aparentemente não precisa
pq já vem assim)
- O segundo passo é saber como referenciar as letras r; o; l; a; como constantes
- O terceiro passo é criar um algorítmo no qual eu consigo comparar cada posição do vetor
com as 4 letras, e assim por diante.
- o quarto passo é criar um vetor de resposta no qual, se tiver a letra é 1, se não tiver 
é 0, e no final, se tiver 1111, é verdade que rola pode ser escrito com os caractéres do input
'''

confirmação = [False, False, False, False]
frase = input ("Digite uma frase aleatória!: ")
for posicao, letra in enumerate(frase):
    print("Elemento na posição", posicao, ":", letra)
    if letra.lower() == "r":
        confirmação[0] = True
    if letra.lower() == "o":
        confirmação[1] = True
    if letra.lower() == "l":
        confirmação[2] = True
    if letra.lower() == "a":
        confirmação[3] = True
if confirmação == [True, True, True, True,]:
    print("A rola está presente nessa frase! Ai que diliça!!")

    '''
    - Confirmação é um vetor, no qual tenho uma condição de 2 alternativas, como são duas, 
    posso usar valores booleanos (True or False), condicionando o acerto a true ou a false
    - "For i, elemento in enumerate(vetor)" é uma estrutura de repetição na qual consigo dizer
    qual é a posição (i) e o valor(elemento) de uma determinada variável (frase) que é um
    vetor, independente do tamanho dele
    - Utiliza-se então uma estrutura de condição, com o .lower(), para avaliar se existe nessa
    sequência (frase) o determinado caractere que se quer identificar, se relacionando então
    com a variável confirmação, que referenciei a cada posição a letra que quero buscar e, se
    todas forem TRUE, então a palavra pode ser escrita com as letras presentes na frase.
    '''