switch = True
while switch == True:
    grau = (input())
    if grau != "":
        grau = int(grau)
        if grau == 360 or  0 <= grau < 90:
            print("Bom Dia!!")
        if 90 <= grau < 180:
            print("Boa Tarde!!")
        if 180 <= grau < 270:
            print("Boa Noite!!")
        if 270 <= grau < 360:
            print("De Madrugada!!")
    else: switch = False
