from django.db import models

# Create your models here.

class Section(models.Model):
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
    measure_choice = models.CharField(choices=measure_units, default="mg", max_length=3, null=False)
    measure = models.IntegerField(null=False)
    total_amount = models.BigIntegerField(null=False)
    category = models.ForeignKey("Section", on_delete=models.PROTECT)   
    
    def __str__(self):
        return f"{self.name} - {self.measure}{self.measure_choice}"
    
class Sale(models.Model):
    id = models.BigAutoField(primary_key=True)
    buyer = models.CharField(null=False, max_length=300)
    products = models.ManyToManyField(Product)
    date = models.DateField(auto_now=True, null=False)
    time = models.TimeField(auto_now=True)
    
    def __str__(self):
        return f"{self.id} - {self.date} {self.time}"
    
class Loss(models.Model):
    id = models.BigAutoField(primary_key=True)
    products = models.ManyToManyField(Product)
    date = models.DateField(auto_now=True, null=False)
    time = models.TimeField(auto_now=True)
    
    def __str__(self):
        return f"{self.id} - {self.date} {self.time}"

class Stock(models.Model):
    id = models.BigAutoField(primary_key=True)
    expire_date = models.DateField(null=False)
    product = models.ManyToManyField(Product)
    
    def __str__(self):
        return f"{self.id} - {self.expire_date}"

class Customer(models.Model):
    id = models.BigAutoField(primary_key=True)
    name = models.CharField(max_length=300, null=False)
    
    def __str__(self):
        return f"{self.name} - {self.id}"