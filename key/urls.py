from django.urls import path, include
from key import views

urlpatterns = [
        path('', include("django.contrib.auth.urls"))
]
