linha = [" 0 "," 0 "," 0 "," 0 "," 0 "," 0 "," 0 "," 0 "]
linhabranca1 = ["wT1","wH1","wB1","wQ","wK","wB2","wH2","wT2"]
linhabranca2 = ["wP1","wP2","wP3","wP4","wP5","wP6","wP7","wP8"]
linhapreta1 = ["bT1","bH1","bB1","bQ","bK","bB2","bH2","bT2"]
linhapreta2 = ["bP1","bP2","bP3","bP4","bP5","bP6","bP7","bP8"]
timebranco = linhabranca1 + linhabranca2
timepreto = linhapreta1 + linhapreta2
tabuleiro = [linha,linha,linha,linha,linha,linha,linha,linha]
turno = 1
player1 = input("\nQual cor vai primeiro?\nPreto ou Branco?: \n")
player2 = input("\nQual cor vai em segundo?\nPreto ou Branco?: \n")
playeratual = player2
if player1.lower() == "branco":
    tabuleiro = [linhabranca1,linhabranca2,linha,linha,linha,linha,linhapreta2,linhapreta1]
if player1.lower() == "preto":
    tabuleiro = [linhapreta1,linhapreta2,linha,linha,linha,linha,linhabranca2,linhabranca1]



def printtabuleiro (tabuleiro : list[list[str]]) -> None:
    for elemento in tabuleiro:
        for element in elemento:
            print(element,end="\t")
        print("\n","\n")
def trocadeplayer (playeratual : str) -> str:
    if playeratual.lower() == "branco":
        playeratual = "preto"
        return playeratual
    else:
        playeratual = "branco"
        return playeratual   
def posatual (peçaescolhida: str, tabuleiro: list[list[str]], linha: list[str]) -> tuple[int,int]:
    line = 0
    column = 0
    while line < len(tabuleiro):
        while column < len(linha):
            element = tabuleiro[line][column]
            if element.lower() == peçaescolhida.lower():
                la = line
                ca = column
                return la,ca
            else: column = column + 1
        column = 0
        line = line + 1
    raise ValueError
def mostrarpeçatual (grupo : list[str]) -> None:
    i = 0
    while i < len(grupo):
        element = grupo[i]
        if  (i+1) == len(grupo) // 2 or i == (len(grupo) - 1):
            print(element,end="\n")
            i = i + 1
        else:
            print(element,end="\t")
            i = i + 1
def validarmov (linhanova: int, colunanova: int, playeratual: str, tabuleiro: list[list[str]]) -> bool:
    if playeratual.lower() == "branco":
        if tabuleiro[linhanova][colunanova][0] == "w" or linhanova > 7 or colunanova > 7:
            return False
        else: 
            return True
    if playeratual.lower() == "preto":
        if tabuleiro[linhanova][colunanova][0] == "b" or linhanova > 7 or colunanova > 7:
            return False
        else: 
            return True
    return True
def mostrarpeças (playeratual : str) -> None:
    if playeratual.lower() == "branco":
        mostrarpeçatual(timebranco)   
    else: mostrarpeçatual (timepreto)




print("\nO primeiro player é o time {}!\n".format(player1))

while turno < 10:
    print("É o turno {}!\n".format(turno))
    playeratual = trocadeplayer(playeratual)
    print("É a vez do time {}!\n".format(playeratual))
    printtabuleiro(tabuleiro)
    mostrarpeças (playeratual)
    pecaescolhida = input("\nEscolha uma peça disponível:\n")
    linhaatual,colunaatual = posatual(pecaescolhida,tabuleiro,linha)    
    isvalid = False
    linhanova = 0
    colunanova = 0
    while isvalid == False:
        linhanova = int(input("\nEscolha a linha da nova posição da peça:\n"))
        colunanova = int(input("\nEscolha a coluna da nova posição da peça:\n"))
        isvalid = validarmov (linhanova, colunanova, playeratual, tabuleiro)
        if isvalid == False:
            print("O movimento é inválido!\n")
    tabuleiro[linhanova][colunanova] = pecaescolhida
    tabuleiro[linhaatual][colunaatual] = "0"
    printtabuleiro(tabuleiro)

        




    turno = int(input("\nQual o turno? :\n"))

