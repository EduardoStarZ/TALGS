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

from django.shortcuts import render
from django.contrib.auth.decorators import login_required
from pages.group import get_user_group

# Create your views here


@login_required
def main(request):
    user = request.user

    user_type = get_user_group(user.username)

    template = 'main.html'

    context = {
            "group": user_type
            }

    return render(request, template, context)


def client(request):
    template = 'client_panel.html'

    context = {}

    return render(request, template, context)


def admin(request):
    template = 'admin_panel.html'

    context = {}

    return render(request, template, context)
