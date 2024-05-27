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


class EstoqueQuerySet(models.QuerySet):

    def due_today(self):
        try:
            today = datetime.datetime.now()
            picked_date = datetime.date(today.year, today.month, today.day)
        except (TypeError, ValueError):
            return self.none()

        objects = self.filter(validade=picked_date)

        for object in objects:
            object.due()

        return objects
