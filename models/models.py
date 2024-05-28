#
#
# models.py
#
# Copyright (c) 2023-2024 (authors)
#
# All rights reserved
#
# TALGS is distributed under the () license, see LICENSE for details
#
#

from django.db import models
from django.db.models import Q
from .queryset import StockQuerySet

class Category(models.Model):
    id = models.BigAutoField(primary_key=True)
    name = models.CharField(null=False, max_length=300)

    def __str__(self):
        return f"{self.name}"


class Product(models.Model):
    measure_units = [
        ("mg", "Miligramas"),
        ("g", "Gramas"),
        ("kg", "Quilogramas"),
        ("ml", "Mililitros"),
        ("L", "Litros"),
    ]

    id = models.BigAutoField(primary_key=True)
    name = models.CharField(max_length=300, null=False)
    price = models.FloatField(null=False)
    choosen_measure = models.CharField(choices=measure_units, default="mg", max_length=3, null=False)
    measure = models.IntegerField(null=False)
    available = models.BooleanField(null=False, default=True)
    total_amount = models.PositiveIntegerField(null=False)
    category = models.ForeignKey("Category", on_delete=models.PROTECT)

    def sync_amount(self):
        stocks = Stock.objects.filter(id_product=self).filter(due=False)

        total = 0
        for stock in stocks:
            total += stock.amount

        self.total_amount = total
        self.save()

    #Função que retorna uma string com o id do produto formatado, para um input disponível
    def available_input(self):
        return f"available-input-{self.id}"

    #Função que retorna uma string com o id do produto formatado, para um input selecionado
    def selected_input(self):
        return f"selected-input-{self.id}"

    #Função que retorna uma string com o id do produto formatado, para um card selecionado
    def selected_card(self):
        return f"selected-card-{self.id}"

    #Função que retorna uma string com o id do produto formatado, para um card disponível
    def available_card(self):
        return f"available-card-{self.id}"

    # Função que retorna um trecho de url para requisições
    def form_input_get_url(self):
        return f"/api/product_form?id={self.id}"

    def __str__(self):
        return f"{self.name} - {self.measure}{self.choosen_measure}"


class Article(models.Model):
    id = models.BigAutoField(primary_key=True)
    id_stock = models.ForeignKey("Stock", on_delete=models.PROTECT, null=False)
    id_purchase = models.ForeignKey("Purchase", on_delete=models.PROTECT, null=False)
    amount = models.PositiveIntegerField(null=False)

    #Retorna o nome do produto referente ao artigo
    def get_art_name(self):
        return Stock.objects.get(id=self.id_stock.id).id_product.name

    def __str__(self):
        return f"{self.id_purchase} - {self.id_stock} / {self.amount}"


class Purchase(models.Model):
    status_codes = [
        (0, 'Na fila'),
        (1, 'Pendente'),
        (2, 'Pronto'),
        (3, 'Completa')
    ]

    id = models.BigAutoField(primary_key=True)
    user = models.ForeignKey("auth.User", null=True, on_delete=models.PROTECT)
    time = models.DateTimeField(auto_now=True, null=False)
    status = models.SmallIntegerField(choices=status_codes, default=0, null=False)

    # Função que retorna os artigos provênientes da instância do produto
    def get_arts(self):
        return Artigo.objects.filter(Q(id_purchase=self.id))

    #Função que retorna uma string com o nome do status, ao inves do número do status
    def status_str(self):
        return self.status_codes[self.status][1]

    #Função que retorna uma string apropriada com o horário do pedido 
    def get_time(self):
        raw_timestamp = str(self.time.astimezone().time())
        raw_timestamp = raw_timestamp.split(":")
        hours = raw_timestamp[0]
        minutes = raw_timestamp[1]

        return str(hours + ':' + minutes)

    #Função que retorna uma string apropriada com a data do pedido
    def get_date(self):
        raw_date = str(self.time.date())
        raw_date = raw_date.split("-")
        year = raw_date[0]
        month = raw_date[1]
        day = raw_date[2]

        return str(day + "/" + month + "/" + year)

    def __str__(self):
        return f"{self.id} - {self.time.astimezone().date()} | {self.time.astimezone().ctime()} - {self.status}"


class Stock(models.Model):
    id = models.BigAutoField(primary_key=True)
    validity = models.DateTimeField(null=False)
    amount = models.SmallIntegerField(null=False)
    id_product = models.ForeignKey("Product", on_delete=models.PROTECT)
    lot_number = models.BigIntegerField(null=True)
    due = models.BooleanField(null=False)
    id_supplier = models.ForeignKey("Supplier", on_delete=models.PROTECT)

    # Queryset customizado do Estoque
    objects = StockQuerySet().as_manager()

    def __str__(self):
        return f"{self.id} - {self.validity}"

    # Função que é executada quando um estoque de produtos está vencido
    def is_due(self):
        self.due = True

        self.save()

        self.id_product.sync_amount()


class Supplier(models.Model):
    id = models.BigAutoField(primary_key=True)
    name = models.CharField(max_length=300, null=False)
    cpf = models.CharField(max_length=11, null=False)
    cnpj = models.CharField(max_length=11, null=True)

    def __str__(self):
        return f"{self.name} - {self.id}"


class Emails(models.Model):
    id = models.BigAutoField(primary_key=True)
    id_user = models.ForeignKey("auth.User", on_delete=models.PROTECT, null=True)
    id_supplier = models.ForeignKey("Supplier", on_delete=models.PROTECT, null=True)
    primary_email = models.EmailField(null=False)
    secondary_email = models.EmailField(null=True)

    def __str__(self):
        return f"{self.id_user} - {self.primary_email}"


class Adress(models.Model):
    states = [
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
    id_user = models.ForeignKey("auth.User", on_delete=models.PROTECT, null=True)
    id_supplier = models.ForeignKey("Supplier", on_delete=models.PROTECT, null=True)
    cep = models.CharField(max_length=8, null=False)
    state = models.CharField(max_length=2, choices=states, null=False)
    town = models.CharField(max_length=300, null=False)
    neighborhood = models.CharField(max_length=300, null=False)
    public_place = models.CharField(max_length=200, null=False)
    complement = models.CharField(max_length=500, null=False)

    def __str__(self):
        return f"{self.state} - {self.town}, {self.neighborhood}"


class Phone(models.Model):

    id = models.BigAutoField(primary_key=True)
    id_user = models.ForeignKey("auth.User", on_delete=models.PROTECT, null=True)
    id_supplier = models.ForeignKey("Supplier", on_delete=models.PROTECT, null=True)
    primary_phone = models.CharField(max_length=11, null=False)
    secondary_phone = models.CharField(max_length=11, null=True)

    def __str__(self):
        return f"{self.id_user}"
