'''
PROBLEMA: Dado um número N de pontos em uma circunferência de um círculo, tem-se que a cada
par de pontos é traçada uma reta que divide a área desse circulo.
1 ponto = 1 área
2 pontos = 2 áreas
3 pontos = 4 áreas
4 pontos = 8 áreas
5 pontos = 16 áreas 
e assim por diante.

RESOLUÇÃO
1- Criar uma váriável para o input da quantidade de pontos (Var: pontos) e criar uma variável
para a quantidade de áreas (Var: area)
2 - fazer uma equação que relacione corretamente pontos e area
3 - printar o output
'''

ponto = int(input())
potencia = ponto - 1
area = 2 ** potencia
print(area)