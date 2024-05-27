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
from .models import Compra, Estoque, Categoria, Produto
from django.db.models import Q
from django.contrib.auth.decorators import login_required
from .behaviour import product_sync


@login_required
def sale_client(request):
    template = 'sale/client.html'

    user = request.user
    sale_query = Compra.objects.filter(Q(usuario=user.id))
    context = {
        'compras': sale_query,
    }

    return render(request, template, context)


@login_required
def create_sale(request):

    product_sync()

    template = "sale/create.html"

    user = request.user

    context = {
        "user": user,
        "products": Produto.objects.filter(~Q(quantidade_total=0)),
        "categorias": Categoria.objects.all()
    }

    return render(request, template, context)
