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

from .models import Produto


def product_sync():
    produtos = Produto.objects.all()

    for produto in produtos:
        produto.sync_amount()
