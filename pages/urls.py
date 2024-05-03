from django.urls import path
from pages import views

urlpatterns = [
    path('', views.main),
    path('client_panel/', views.client)
]