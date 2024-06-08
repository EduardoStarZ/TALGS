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

from django.shortcuts import render, redirect
from .models import Purchase, Stock, Category, Product, Article
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

    due_stocks = Stock.objects.due_today()
    normal_stocks = Stock.objects.not_due_today()

    for stock in due_stocks:
        stock.is_due()

    for stock in normal_stocks:
        stock.is_not_due()

    template = "sale/create.html"

    user = request.user

    context = {
        "user": user,
        "Products": Product.objects.filter(~Q(total_amount=0) & Q(available=True)),
        "Categories": Category.objects.all()
    }

    if request.method == 'POST':
        form = dict(request.POST)
        form.pop('csrfmiddlewaretoken')

        print(form)

        print(len(form))

        keys = []

        for key in form.keys():
            keys.insert(len(keys), key.replace('amount-', '').replace('id-', ''))

        temp_dict = dict.fromkeys(keys)

        keys = list(temp_dict)

        if keys.__len__() == 0:
            return redirect('/')

        sale = Purchase(user=user)

        sale.save()

        print(keys)
        print(keys.__len__())

        for value in keys:
            prod_id = form.get('id-'+str(value))[0]
            amount = form.get('amount-'+str(value))[0]

            selected_stock = Stock.objects.not_due_today().filter(id_product=prod_id).order_by('validity__date')

            stock_instance = Stock.objects.get(id=selected_stock[0].id)

            new_article = Article(id_purchase=sale, id_stock=stock_instance, amount=amount)

            stock_instance.amount -= int(amount)

            stock_instance.save()

            new_article.save()

        return redirect('/')



    return render(request, template, context)


@login_required
def product_selected_card(request):

    template = "sale/selected_card.html"

    if request.method == 'GET':
        attributes = dict(request.GET)
        product = Product.objects.get(id=int(attributes.get('id')[0]))

    context = {
        'product': product
            }

    return render(request, template, context)


@login_required
def product_available_card(request):

    template = "sale/available_card.html"

    if request.method == 'GET':
        attributes = dict(request.GET)

        id = attributes.get('filtro')

        excludents = attributes.get('exclude')

        product = Product.objects.all().filter(~Q(total_amount=0) & Q(available=False))

        if id[0] != 'none':
            product = Product.objects.is_from(id[0])

        if 'exclude' in attributes.keys():
            for value in excludents:
                product = product.exclude(id=value)

    context = {
        'Products': product
            }

    return render(request, template, context)
