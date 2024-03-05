from django.urls import path
from pages import views

urlpatterns = [
    path("", views.frame, name='frame'),
    path("home/", views.home, name='main'),
    path("admin/", views.admin, name='admin')
]