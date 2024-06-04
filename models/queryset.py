#
#
# queryset.py
#
# Copyright (c) 2023-2024 (authors)
#
# All rights reserved
#
# TALGS is distributed under the () license, see LICENSE for details
#
#

from django.db import models
import datetime

class StockQuerySet(models.QuerySet):

    # Queryset method that checks if the due date is the same as today
    def due_today(self):
        try:
            today = datetime.datetime.now()
        except (TypeError, ValueError):
            return self.none()

        objects = self.filter(validity__date__lte=today)

        return objects

    # Queryset method that checks if the due date is the same as today
    def not_due_today(self):
        try:
            today = datetime.datetime.now()
        except (TypeError, ValueError):
            return self.none()

        objects = self.filter(validity__date__gte=today)

        return objects


class ProductQuerySet(models.QuerySet):
    def is_from(self, category_id):

        return self.filter(category__id=category_id)
