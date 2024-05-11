from django.shortcuts import render
from .models import Compra, Estoque, Produto
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
        "products": Estoque.objects.filter(
            ~Q(quantidade=0) & Q(disponivel=True)
            )
    }

    return render(request, template, context)
