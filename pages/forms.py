from django.forms import ModelForm

from models.models import *

class SectionForm(ModelForm):
    class Meta:
        model = Section
        fields = '__all__'