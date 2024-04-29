from django.shortcuts import render
from django.contrib.auth.decorators import login_required

# Create your views here

@login_required
def main(request):
    template = 'main.html'
    
    context = {}
    
    return render(request, template, context)