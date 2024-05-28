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
from .models import Purchase, Stock, Category, Product
from django.db.models import Q
from django.contrib.auth.decorators import login_required
from .behaviour import product_sync


@login_required
def sale_client(request):
    template = 'sale/client.html'

    user = request.user
    context = {
        'Purchases': Purchase.objects.filter(Q(user=user.id)),
    }

    return render(request, template, context)


@login_required
def create_sale(request):

    product_sync()

    stocks = Stock.objects.due_today()

    for stock in stocks:
        stock.due()

    template = "sale/create.html"

    user = request.user

    context = {
        "user": user,
        "Products": Product.objects.filter(~Q(total_amount=0) & Q(available=True)),
        "Categories": Category.objects.all()
    }

    return render(request, template, context)


@login_required
def product_form(request):

    template = "product_form.html"

    if request.method == 'GET':
        attributes = dict(request.GET)
        product = Product.objects.get(id=int(attributes.get('id')[0]))

    context = {
        'product': product
            }

    return render(request, template, context)
