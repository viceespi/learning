nome = "Popocão"
valor = input (nome + ", quanto você recebeu por ser chato assim?: ")
if valor.isdigit() == True: respostavalorcorreto
else: print ("É sim ou não porra")
respostavalorcorreto = input (nome + ", esse valor tá certo mesmo?: ")
while respostavalorcorreto.lower() == "não":
    valor = input (nome + ", quanto você recebeu então porra: ") 
    respostavalorcorreto = input (nome + ", esse valor tá certo mesmo?: ")
if respostavalorcorreto.lower() == "sim":
    decisão = input (nome + ", o quê você quer fazer então?: ")
else: print ("É sim ou não porra")
# Vou deixar aqui pra um dia terminar certinho!



nome = input("Olá! \n Qual o seu nome?: ")
valor = int(input ("Oi," + nome + ", quanto você recebeu por ser chato assim???: "))
valorcorreto = input ("{}, esse valor está correto? :").format(nome)
def corrigirvalor (valorcorreto: str):
    while valorcorreto.lower() != "sim":
       
if valorcorreto.lower() == "não":
     corrigirvalor (valorcorreto)

