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
from . import views

urlpatterns = [
    path('sale/client', views.sale_client),
    path('sale/create', views.create_sale),
    path('sale/selected_card', views.product_selected_card),
    path('sale/available_card', views.product_available_card)
]
