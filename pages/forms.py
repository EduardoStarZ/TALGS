from django.forms import ModelForm

from models.models import *

class SaleForm(ModelForm):
    class Meta:
        model = Compra
        fields = '__all__'
        
class Form(ModelForm):
    class Meta:
        model = Compra
        fields = '__all__'
