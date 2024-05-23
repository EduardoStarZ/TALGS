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
from pages import views

urlpatterns = [
    path('', views.main),
    path('client_panel/', views.client)
]

