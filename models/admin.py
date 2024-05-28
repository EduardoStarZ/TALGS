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

admin.site.register(models.Category)
admin.site.register(models.Product)
admin.site.register(models.Purchase)
admin.site.register(models.Article)
admin.site.register(models.Emails)
admin.site.register(models.Stock)
admin.site.register(models.Adress)
admin.site.register(models.Phone)
admin.site.register(models.Supplier)
