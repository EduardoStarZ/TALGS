from django.urls import path
from django.contrib.auth import views

urlpatterns = [
        path('login/', views.LoginView.as_view()),
        path('logout/', views.LogoutView.as_view()),
        path('change_password/', views.PasswordChangeView.as_view())
]
