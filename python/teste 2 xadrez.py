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


def printtabuleiro (tabuleiro : list[list[str]]):
    for elemento in tabuleiro:
        for element in elemento:
            print(element,end="\t")
        print("\n","\n")
def trocadeplayer (playeratual : str):
    if playeratual.lower() == "branco":
        playeratual = "preto"
        return playeratual
    else:
        playeratual = "branco"
        return playeratual
    
print("\nO primeiro player é o time {}!\n".format(player1))

while turno < 10:
    print("É o turno {}!\n".format(turno))
    playeratual = trocadeplayer(playeratual)
    print("É a vez do time {}!\n".format(playeratual))
    printtabuleiro(tabuleiro)
    if playeratual.lower() == "branco":
        print(timebranco,end="\t")
        input("Escolha uma peça:\n")
