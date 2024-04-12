# Visão da aplicação

## Objetivo principal:

O T.A.L.G.S. procura oferecer uma interface de gerenciamento de vendas e estoques de loja,
de maneira simples, eficaz e rápida, permitindo assim um maior controle sobre o fluxo de 
vendas da loja, sobre o capital que entra e saí da loja e sobre os estoques dos produtos 
com seu sistema de gerenciamento de validade.
Existem diversos sistemas que possuem pelo menos dois das características acima,
porém muitos poucos possuem todos estes e tem a mais fundamental propriedade que é a gratuidade como o nosso.

## Metas:

#### Curto prazo:
- Aumentar a velocidade de atendimento da loja.
- Diminuir a carga do trabalho do atendente físico da loja.
- Garantir uma maior precisão do fluxo de capital da loja nos espaços onde o sistema atua, sem aberturas para erros críticos.

#### Longo prazo: 

- Melhorar a experiência de atendimento do cliente final.
- Aumentar o entendimento sobre o sistema para o usuário deste.
- Diminuir o tempo necessário total para gerir a loja física.
- Estabelecer uma posição de destaque do T.A.L.G.S. sobre o pequeno universo de sistemas de gerenciamento de vendas.


## Funcionalidades principais:

1. Interface intuitiva: 

`
    É uma boa prática tornar interfaces gráficas simples, intuitivas e com o mínimo possivel de elementos visíveis, mas com o máximo de funcionalidades disponíveis. Por isso, o T.A.L.G.S. conta com uma interface minimalista e de fácil de utilização.
` 

2. Bancos de dados local e remoto: 

`
    O sistema conta com uma funcionalidade que permite ao servidor conectar-se com dois bancos de dados relativamente simultaneamente. O primeiro banco de dados é o local, é a partir dele que as vendas são declaradas, onde o sistema acessa os produtos disponíveis e os revela ao usuário. Já o segundo sistema é remoto, é nele onde se centralizam todos os dados existentes do usuário para se em caso de falha catastrófica, ser disponibilizado um backup integral dos dados do sistema.
` 

3. Rastreamento de validade e quantidades de produtos: 

`
    Foi implementado uma funcionalidade que cria alertas para quando produtos do estoque estão próximos do vencimento ou próximos do esgotamento no estoque. Esta funcionalidade é customizável pelo próprio usuário, podendo escolher quando o sistema deve alertar sobre a validade de um produto em relação a sua data de vencimento ou sobre a quantidade de um produto se esgotando.
`

4. Funcionalidade CRUD para produtos:

`
    CRUD é uma sigla para "create, read, update, delete". É utilizado para descrever sistemas que possuem funcionalidades que, como o nome sugere, criam, leêm, alteram e excluem objetos de seus sistemas. No caso do T.A.L.G.S. essa funcionalidade aparece no gerenciamento de produtos, onde um usuário pode realizar estas operações em cima de seus produtos, sem em momento algum alterar transações passadas.
`