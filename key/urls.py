#
#
# urls.py
#
# Copyright (c) 2023-2024 (authors)
#
# All rights reserved
#
# TALGS is distributed under the () license, see LICENSE for details
#
#

from django.urls import path
from django.contrib.auth import views as built_in
from key import views

urlpatterns = [
    path('register/', views.create_account, name='create-user'),
    path('login/', built_in.LoginView.as_view(), name='login'),
    path('logout/', built_in.LogoutView.as_view(), name='logout'),
    path('password/change/', built_in.PasswordChangeView.as_view()),
    path('password/change/done/',
         built_in.PasswordChangeDoneView.as_view(),
         name='password_change_done'),
]
