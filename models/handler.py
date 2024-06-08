#
# handler.py
#
# Copyright (c) 2023-2024 (authors)
#
# All rights reserved
#
# TALGS is distributed under the () license, see LICENSE for details
#
#

from .models import Purchase, Stock, Article


def purchase_handler(form, user):
    print(form)
    keys = []

    for key in form.keys():
        keys.insert(len(keys), key.replace('amount-', '').replace('id-', ''))

    temp_dict = dict.fromkeys(keys)

    keys = list(temp_dict)

    if keys.__len__() == 0:
        return False

    sale = Purchase(user=user)

    sale.save()

    print(keys)

    for value in keys:
        prod_id = form.get('id-'+str(value))[0]
        amount = form.get('amount-'+str(value))[0]

        selected_stock = Stock.objects.not_due_today().filter(id_product=prod_id).order_by('validity__date')

        stock_instance = Stock.objects.get(id=selected_stock[0].id)

        new_article = Article(id_purchase=sale, id_stock=stock_instance, amount=amount)

        stock_instance.amount -= int(amount)

        stock_instance.save()

        new_article.save()

    return True
