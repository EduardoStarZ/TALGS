#
#
# behaviour.py
#
# Copyright (c) 2023-2024 (authors)
#
# All rights reserved
#
# TALGS is distributed under the () license, see LICENSE for details
#
#

from .models import Product


def product_sync():
    products = Product.objects.all()

    for product in products:
        product.sync_amount()
