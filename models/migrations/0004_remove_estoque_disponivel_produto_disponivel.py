# Generated by Django 5.0.6 on 2024-05-27 22:44

from django.db import migrations, models


class Migration(migrations.Migration):

    dependencies = [
        ('models', '0003_rename_id_produto_estoque_id_produto'),
    ]

    operations = [
        migrations.RemoveField(
            model_name='estoque',
            name='disponivel',
        ),
        migrations.AddField(
            model_name='produto',
            name='disponivel',
            field=models.BooleanField(default=True),
        ),
    ]
