from django.urls import path, include
from django.contrib.auth import views

urlpatterns = [
        path('', views.LoginView.as_view()),
        path('logout/', views.LogoutView.as_view()),
        path('change_password', views.PasswordChangeView.as_view())
]
