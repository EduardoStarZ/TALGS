from django.shortcuts import redirect, render
from django.contrib.auth.decorators import login_required
from django.contrib import auth
from django.contrib import messages


def login(request):
    template = 'login.html'
    context = {}
        
    return render(request, template, context)
    
@login_required
def password_change(request):
    template = 'password_change_form.html'
    context = {}
    
    return render(request, template, context)

@login_required
def password_change_done(request):
    template = 'password_change_done.html'
    context = {}
    
    return render(request, template, context)