from django.contrib import admin
from . import models

# Register your models here.

admin.site.register(models.Section)
admin.site.register(models.Product)
admin.site.register(models.Sale)
admin.site.register(models.Loss)
admin.site.register(models.Customer)
admin.site.register(models.Stock)