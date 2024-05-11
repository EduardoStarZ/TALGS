from django.shortcuts import render
from .models import Compra, Artigo
from django.db.models import Q
from django.contrib.auth.decorators import login_required

# Create your views here.
    
@login_required
def sale_client(request):
    template = 'sale/client.html'

    user = request.user
    sale_query = Compra.objects.filter(Q(usuario=user.id))
    context = {
        'compras': sale_query,
    }
    
    return render(request, template, context)
