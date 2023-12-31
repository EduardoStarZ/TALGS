from django.shortcuts import render

def login(request):
        template = 'login.html'
        context = {}
        
        return render(request, template, context)

def logout(request):
        template = 'logout.html'
        context = {}
        
        return render(request, template, context)