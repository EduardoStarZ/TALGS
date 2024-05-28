#
#
# admin.py
#
# Copyright (c) 2023-2024 (authors)
#
# All rights reserved
#
# TALGS is distributed under the () license, see LICENSE for details
#
#

from django.contrib import admin
from .models import Category, Product, Purchase, Article, Emails, Stock, Adress, Phone, Supplier

# Register your models here.

admin.site.register(Category)
admin.site.register(Product)
admin.site.register(Purchase)
admin.site.register(Article)
admin.site.register(Emails)
admin.site.register(Stock)
admin.site.register(Adress)
admin.site.register(Phone)
admin.site.register(Supplier)
