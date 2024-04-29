from django.urls import path
from django.contrib.auth import views

urlpatterns = [
        path('login/', views.LoginView.as_view()),
        path('logout/', views.LogoutView.as_view()),
        path('password/change/', views.PasswordChangeView.as_view()),
        path('password/change/done/', views.PasswordChangeDoneView.as_view(), name='password_change_done'),
]
