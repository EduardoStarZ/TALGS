from django.shortcuts import render
from .models import Compra, Artigo, Estoque, Produto
from django.db.models import Q
from django.contrib.auth.decorators import login_required

# Create your views here.
    
def behaviour():
    print("i started executing")
    produtos = Produto.objects.all()

    for produto in produtos:
        produto.sync_amount()

    print("i finished executing")

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

    behaviour()

    template = "sale/create.html"

    user = request.user

    context = {
            "products": Estoque.objects.filter(~Q(quantidade=0))
    }

    return render(request, template, context)
