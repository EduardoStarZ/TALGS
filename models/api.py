from django.shortcuts import render
from . import forms

# Create your views here.


def main(request):
    form = forms.ProductForm
    template = 'index.html'
    context = {
        'TestForm': form
    }
    return render(request, template, context)
