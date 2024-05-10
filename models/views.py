from django.shortcuts import render
from .models import Compra
from django.db.models import Q
from django.contrib.auth.decorators import login_required

# Create your views here.
    
@login_required
def sale_client(request):
    template = 'sale/client.html'
        
    user = request.user
    query = Compra.objects.filter(Q(usuario=user.id))
    
    status_codes = ["Na fila", "Pendente", "Pronto", "Completa"]
    
    status_names = []
    
    for x in query:
        status_names.append(status_codes[x.status])
    
    context = {
        'compras': query,
    }
    
    return render(request, template, context)