from django.urls import path
from pages import views

urlpatterns = [
    path("", views.frame),
    path("home/", views.home),
    path("admin/", views.admin)
]