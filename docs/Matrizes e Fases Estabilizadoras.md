Na simulação de circuitos quânticos usando operações de Clifford, a representação estabilizadora é uma ferramenta poderosa que permite uma simulação eficiente. Vamos aprofundar o que são a matriz estabilizadora e o vetor estabilizador, e porque eles são representados como inteiros.

### O que são a Matriz Estabilizadora e o Vetor Estabilizador?

#### Matriz Estabilizadora (G)
A matriz estabilizadora, frequentemente denotada por \(G\), é uma matriz binária que representa os operadores de Pauli que estabilizam o estado quântico. Em termos simples, operadores estabilizadores são operadores que, quando aplicados a um estado quântico, deixam o estado inalterado.

Sendo $E$ a matriz resultante de um dos conjuntos estabilizadores, compostos por matrizes de Pauli, e $\Ket{\psi}$ um estado qualquer, a matriz $E$ é qualificada como estabilizadora se, e somente se, cumprir a seguinte equação:
$$
E \Ket{\psi} = \Ket{\psi}
$$
Cada linha da matriz estabilizadora $G$ representa um operador de Pauli, onde cada operador é descrito pelos coeficientes das bases de Pauli (I, X, Y, Z). Como estamos lidando com operações de Clifford, que preservam a estrutura dos operadores de Pauli, podemos representar cada operador de Pauli usando bits:

- 1 bit para o coeficiente de $X$
- 1 bit para o coeficiente de $Z$

Para um sistema com $n$ qubits, $G$ terá $2n$ linhas (para acomodar os operadores de Pauli $X$ e $Z$ para cada qubit) e $2n$ colunas.

#### Vetor Estabilizador (F)
O vetor estabilizador $F$ é um vetor que representa a fase de cada operador estabilizador. Em operações de Clifford, as fases dos operadores de Pauli podem ser $+1$, $-1$, $+i$ ou $-i$ que são representadas como inteiros:

- 0 para $+1$
- 1 para $+i$
- 2 para $-1$
- 3 para $-i$

### Por que eles são Inteiros?
A matriz estabilizadora $G$ e o vetor estabilizador $F$ são inteiros (ou mais precisamente,  a $G$ possuí valores binários) porque estamos representando a presença e ausência de coeficientes das bases de Pauli $X$ e $Z$ e as fases de maneira compacta e eficiente:

- **Inteiros Binários:** Cada elemento de \(G\) e \(F\) é um bit (0 ou 1), representando a presença (1) ou ausência (0) de um termo específico de Pauli e a fase \((+1\) ou \(-1)\).
- **Eficiência:** Usar bits para representar operadores de Pauli e suas fases torna as operações de manipulação de operadores (como CNOT, Hadamard e S) extremamente eficientes em termos de tempo de execução e uso de memória.

### Exemplo Detalhado

Vamos ilustrar isso com um exemplo para um sistema de 2 qubits.

#### Estado Inicial |00⟩
Para um estado inicial \(|00⟩\), os estabilizadores são:

- $Z_1 |00⟩ = |00⟩$
- $Z_2 |00⟩ = |00⟩$

Isso é representado por:

```
G = [1 0 | 0 0]
    [0 1 | 0 0]
    [0 0 | 1 0]
    [0 0 | 0 1]

F = [0, 0, 0, 0]
```

Aqui, cada linha de \(G\) representa um operador de Pauli \(X\) ou \(Z\) para um dos qubits.

### Aplicação de Portas

1. **Hadamard em Qubit 0**
   - A porta Hadamard troca X e Z, então:

```
G = [0 0 | 1 0]
    [0 1 | 0 0]
    [1 0 | 0 0]
    [0 0 | 0 1]
```

2. **Fase (S) em Qubit 1**
   - A porta S mapeia Z -> Z e X -> Y (com \(Y = iXZ\)):

```
G = [0 0 | 1 0]
    [0 1 | 0 0]
    [1 0 | 0 0]
    [0 0 | 1 1]
```
### Revisão das Portas Clifford e Fases

1. **Porta Hadamard (H)**:
   - Troca $(X)$ e $(Z)$: $(H \cdot X \cdot H = Z)$ e $(H \cdot Z \cdot H = X)$.
   - As fases dos estabilizadores não são alteradas pela porta Hadamard.

2. **Porta de Fase (S)**:
   - Mapeia $(X \rightarrow Y = iXZ)$ e $(Z \rightarrow Z):$
     - A aplicação da porta S introduz uma fase de \(i\) em \(X\), que pode ser representada no vetor de fases \(F\).

3. **Porta CNOT**:
   - Mapeia $(X \rightarrow XX)$ e $(Z \rightarrow IZ)$:
     - As fases dos estabilizadores não são diretamente alteradas pela porta CNOT.