'''
Dado uma lista desordada de números, com N elementos, ordene a lista em ordem crescente
utilizando 3 métodos diferentes. Cada método deve ser uma função diferente.

Minha ideia de solução é que eu pegue cada par de números e compare se um é menor que o outro, 
e então faça uma troca entre eles dois, fazendo isso para todos os elementos.
Já posso cortar um passo fazendo com que a primeira casa tenha o valor mínimo e a ultima casa
tenha o valor máximo. 
    - Para isso preciso:
        - Saber o valor mínimo
            - utilizar a função min()
        - Saber qual é a casa do min
            - Utilizar um iterador que me dê o valor do elemento e a posição desse elemento
                - Preciso saber qual é o ultimo endereço da minha lista
                    - primeira casa = vetor[0]. Sempre padrão
                    - ultima casa = vetor[len(vetor)-1]
                - Iterar e printar o valor e a casa para cada elemento que ele passa
        - Guardar o valor da primeira casa em outra váriável, já que a troca do valor da casa
        0 para o min acaba sobrescrevendo o valor antigo da casa 0
            - Criar uma variavel e atribuir o valor de [0]
        - Reescrever o valor de [0] para valor min
        - reescrever o valor da casa do min com o valor da [0]
        - Repetir todos esses passos mas agora com o valor máximo
Feito isso, preciso começar a desenvolver o algoritmo de comparação, e depois o algorítmo troca
    - Para o algorítmo de comparação, preciso pegar cada casa e comparar com a seguinte, e depois
    fazer o mesmo com as casas seguintes.
        - Pensei em utilizar A e B e fazer uma operação de A < B, retornando o valor True que vai
        ser utilizado depois para a função de troca.
        - Como é uma validação que precisa andar pelos elementos posso tentar fazer o seguinte:
            - while B != len(num):
                - A = 0
                - B = 1
                - VA = num[A]
                - VB = num[B]
                - VA < VB
                    - if False:
                        VT = VA
                        VA = VB
                        num(B) = VT
                        A = 0
                        B = 1
                - A = A + 1
                - B = B + 1
    - Após o sucesso no algorítmo de comparação, preciso fazer a função de troca de casa de 
    acordo com o resultado da comparação. Se comp = true, não muda de casa, se False, troca
    de casa
        - Para trocar de casa, é preciso fazer algo similar com o primeiro problema de trocar
        os valores min e max de posição, que é:
            - Criar uma variável temporária (VT) para armazenar o valor do maior valor que vai
            ser substituido (VA)
            - Fazer a substituição do maior valor (VA) pelo menor valor (VB), resultado em (B) e (B)
            - Fazer a substituição do valor na casa B por VT
'''
numero = [73, 21, 94, 42, 10, 87, 58, 35, 66, 5, 1]
print(numero)
a = 0
b = 1
while b < len(numero):
    va = numero[a]
    vb = numero[b]
    comparação = va < vb
    if comparação == False:
        vt = va
        numero[a] = vb
        numero[b] = vt
        a = 0
        b = 1
    else: 
        a = a + 1
        b = b + 1
print(numero)