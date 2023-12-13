from django.shortcuts import render

# Create your views here.


def frame(request):
    template = "frame.html"
    context = {}
    
    return render(request, template, context)

def home(request):
    template = 'home.html'
    context = {}
    
    return render(request, template, context)

def admin(request):
    template = 'admin.html'
    context = {}
    
    return render(request, template, context)