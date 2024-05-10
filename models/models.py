from django.db import models
from django.utils import timezone

class Categoria(models.Model):
    id = models.BigAutoField(primary_key=True)
    nome = models.CharField(null=False, max_length=300)
    
    def __str__(self):
        return f"{self.nome}"




class Produto(models.Model):
    unidades_medida = [
        ("mg", "Miligramas"),
        ("g", "Gramas"),
        ("kg", "Quilogramas"),
        ("ml", "Mililitros"),
        ("L", "Litros"),
    ]
    
    id = models.BigAutoField(primary_key=True)
    nome = models.CharField(max_length=300, null=False)
    preço = models.FloatField(null=False)
    unidade_escolhida = models.CharField(choices=unidades_medida, default="mg", max_length=3, null=False)
    medida = models.IntegerField(null=False)
    quantidade_total = models.PositiveIntegerField(null=False)
    categoria = models.ForeignKey("Categoria", on_delete=models.PROTECT)   
    
    def __str__(self):
        return f"{self.nome} - {self.medida}{self.unidade_escolhida}"


class Artigo(models.Model):
    id = models.BigAutoField(primary_key=True)
    id_estoque = models.ForeignKey("Estoque", on_delete=models.PROTECT, null=False)
    id_compra = models.ForeignKey("Compra", on_delete=models.PROTECT, null=False)
    quantidade = models.PositiveIntegerField(null=False)
    
    def __str__(self):
        return f"{self.id_compra} - {self.id_estoque} / {self.quantidade}"



class Compra(models.Model):
    escolhas_status = [
        (0, 'Na fila'),
        (1, 'Pendente'),
        (2, 'Pronto'),
        (3, 'Completa')
    ]
    
    id = models.BigAutoField(primary_key=True)
    usuario = models.ForeignKey("auth.User", null=True, on_delete=models.PROTECT)
    horario = models.DateTimeField(auto_now=True, null=False)
    status = models.SmallIntegerField(choices=escolhas_status, default=0, null=False)
    
    def status_str(self):
        return self.escolhas_status[self.status][1]
    
    def __str__(self):
        return f"{self.id} - {self.horario.astimezone().date()} | {self.horario.astimezone().ctime()} - {self.status}"



class Estoque(models.Model):
    id = models.BigAutoField(primary_key=True)
    validade = models.DateTimeField(null=False)
    quantidade = models.SmallIntegerField(null=False)
    id_Produto = models.ForeignKey("Produto", on_delete=models.PROTECT)
    numero_lote = models.BigIntegerField(null=True)
    vencido = models.BooleanField(null=False)
    disponivel = models.BooleanField(null=False)
    id_fornecedor = models.ForeignKey("Fornecedor", on_delete=models.PROTECT)
    
    def __str__(self):
        return f"{self.id} - {self.validade}"



class Fornecedor(models.Model):
    id = models.BigAutoField(primary_key=True)
    name = models.CharField(max_length=300, null=False)
    cpf = models.CharField(max_length=11, null=False)
    cnpj = models.CharField(max_length=11, null=True)
    
    
    def __str__(self):
        return f"{self.name} - {self.id}"


class Emails(models.Model):
    id = models.BigAutoField(primary_key=True)
    id_usuario = models.ForeignKey("auth.User", on_delete=models.PROTECT, null=True)
    id_fornecedor = models.ForeignKey("Fornecedor", on_delete=models.PROTECT, null=True)
    email_primario = models.EmailField(null=False)
    email_secundario = models.EmailField(null=True)
    
    def __str__(self):
        return f"{self.id_usuario} - {self.email_primario}"
        


class Endereço(models.Model):
    estados = [
    ("AC", "Acre"),
    ("AL", "Alagoas"),
    ("AP", "Amapá"),
    ("AM", "Amazonas"),
    ("BA", "Bahia"),
    ("CE", "Ceará"),
    ("DF", "Distrito Federal"),
    ("ES", "Espírito Santo"),
    ("GO", "Goiás"),
    ("MA", "Maranhão"),
    ("MT", "Mato Grosso"),
    ("MS", "Mato Grosso do Sul"),
    ("MG", "Minas Gerais"),
    ("PA", "Pará"),
    ("PB", "Paraíba"),
    ("PR", "Paraná"),
    ("PE", "Pernambuco"),
    ("PI", "Piauí"),
    ("RJ", "Rio de Janeiro"),
    ("RN", "Rio Grande do Norte"),
    ("RS", "Rio Grande do Sul"),
    ("RO", "Rondônia"),
    ("RR", "Roraima"),
    ("SC", "Santa Catarina"),
    ("SP", "São Paulo"),
    ("SE", "Sergipe"),
    ("TO", "Tocantins")
    ]
    
    
    id = models.BigAutoField(primary_key=True)
    id_usuario = models.ForeignKey("auth.User", on_delete=models.PROTECT, null=True)
    id_fornecedor = models.ForeignKey("Fornecedor", on_delete=models.PROTECT, null=True)
    cep = models.CharField(max_length=8, null=False)
    
    estado = models.CharField(max_length=2, choices=estados, null=False)
    cidade = models.CharField(max_length=300, null=False)
    bairro = models.CharField(max_length=300, null=False)
    logradouro = models.CharField(max_length=200, null=False)
    complemento = models.CharField(max_length=300, null=False)
    
    def __str__(self):
        return f"{self.estado} - {self.cidade}, {self.bairro}"
    
class Telefone(models.Model):
    
    id = models.BigAutoField(primary_key=True)
    id_usuario = models.ForeignKey("auth.User", on_delete=models.PROTECT, null=True)
    id_fornecedor = models.ForeignKey("Fornecedor", on_delete=models.PROTECT, null=True)
    numero_primario = models.CharField(max_length=11, null=False)
    numero_secundario = models.CharField(max_length=11, null=True)
    
    def __str__(self):
        return f"{self.id_usuario}"

