#
#
# forms.py
#
# Copyright (c) 2023-2024 (authors)
#
# All rights reserved
#
# TALGS is distributed under the () license, see LICENSE for details
#
#

from django.forms import ModelForm
from models.models import Purchase


class PurchaseForm(ModelForm):
    class Meta:
        model = Purchase
        fields = '__all__'
