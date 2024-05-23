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
from . import models

# Register your models here.

admin.site.register(models.Categoria)
admin.site.register(models.Produto)
admin.site.register(models.Compra)
admin.site.register(models.Artigo)
admin.site.register(models.Emails)
admin.site.register(models.Estoque)
admin.site.register(models.Endereço)
admin.site.register(models.Telefone)
admin.site.register(models.Fornecedor)
