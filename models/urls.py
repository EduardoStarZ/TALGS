from django.urls import path
from . import views

urlpatterns = [
    path('sale/client', views.sale_client),
    path('sale/create', views.create_sale)
]
