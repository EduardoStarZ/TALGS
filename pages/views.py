from django.shortcuts import render

# Create your views here

def main(request):
    template = 'main.html'
    
    context = {}
    
    return render(request, template, context)