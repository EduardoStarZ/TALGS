#
#
# views.py
#
# Copyright (c) 2023-2024 (authors)
#
# All rights reserved
#
# TALGS is distributed under the () license, see LICENSE for details
#
#

from django.shortcuts import redirect, render
from django.contrib.auth.decorators import login_required
from key.group import group_user
from key.forms import SignupForm


def login(request):
    template = 'login.html'

    context = {}

    return render(request, template, context)


def create_account(request):
    template = 'create_account.html'

    if request.method == 'POST':
        form = SignupForm(request.POST)
        if form.is_valid():
            form.save()
            group_user(form['username'].value())
            return redirect('login')
    else:
        form = SignupForm()

    context = {
            'form': form
            }

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

