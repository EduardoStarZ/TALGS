from .models import Produto


def product_sync():
    produtos = Produto.objects.all()

    for produto in produtos:
        produto.sync_amount()
