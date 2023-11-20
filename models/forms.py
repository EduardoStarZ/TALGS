from . import models
from django.forms import ModelForm

class ProductForm(ModelForm):
    class Meta:
        model = models.Product
        fields = '__all__'
        
class SaleForm(ModelForm):
    class Meta:
        model = models.Sale
        fields = '__all__'
        
class LossForm(ModelForm):
    class Meta:
        model = models.Loss
        fields = '__all__'