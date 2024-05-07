# Generated by Django 5.0.4 on 2024-05-07 21:52

import django.db.models.deletion
from django.conf import settings
from django.db import migrations, models


class Migration(migrations.Migration):

    initial = True

    dependencies = [
        migrations.swappable_dependency(settings.AUTH_USER_MODEL),
    ]

    operations = [
        migrations.CreateModel(
            name='Categoria',
            fields=[
                ('id', models.BigAutoField(primary_key=True, serialize=False)),
                ('nome', models.CharField(max_length=300)),
            ],
        ),
        migrations.CreateModel(
            name='Estoque',
            fields=[
                ('id', models.BigAutoField(primary_key=True, serialize=False)),
                ('validade', models.DateTimeField()),
                ('quantidade', models.SmallIntegerField()),
                ('numero_lote', models.BigIntegerField(null=True)),
                ('vencido', models.BooleanField()),
                ('disponivel', models.BooleanField()),
            ],
        ),
        migrations.CreateModel(
            name='Fornecedor',
            fields=[
                ('id', models.BigAutoField(primary_key=True, serialize=False)),
                ('name', models.CharField(max_length=300)),
                ('cpf', models.CharField(max_length=11)),
                ('cnpj', models.CharField(max_length=11, null=True)),
            ],
        ),
        migrations.CreateModel(
            name='Compra',
            fields=[
                ('id', models.BigAutoField(primary_key=True, serialize=False)),
                ('horario', models.DateTimeField(auto_now=True)),
                ('status', models.CharField(choices=[(0, 'Na fila'), (1, 'Pendente'), (2, 'Pronto'), (3, 'Completa')], default='Na fila', max_length=10)),
                ('usuario', models.ForeignKey(null=True, on_delete=django.db.models.deletion.PROTECT, to=settings.AUTH_USER_MODEL)),
            ],
        ),
        migrations.CreateModel(
            name='Artigo',
            fields=[
                ('id', models.BigAutoField(primary_key=True, serialize=False)),
                ('quantidade', models.PositiveIntegerField()),
                ('id_compra', models.ForeignKey(on_delete=django.db.models.deletion.PROTECT, to='models.compra')),
                ('id_estoque', models.ForeignKey(on_delete=django.db.models.deletion.PROTECT, to='models.estoque')),
            ],
        ),
        migrations.AddField(
            model_name='estoque',
            name='id_fornecedor',
            field=models.ForeignKey(on_delete=django.db.models.deletion.PROTECT, to='models.fornecedor'),
        ),
        migrations.CreateModel(
            name='Endereço',
            fields=[
                ('id', models.BigAutoField(primary_key=True, serialize=False)),
                ('cep', models.CharField(max_length=8)),
                ('estado', models.CharField(choices=[('AC', 'Acre'), ('AL', 'Alagoas'), ('AP', 'Amapá'), ('AM', 'Amazonas'), ('BA', 'Bahia'), ('CE', 'Ceará'), ('DF', 'Distrito Federal'), ('ES', 'Espírito Santo'), ('GO', 'Goiás'), ('MA', 'Maranhão'), ('MT', 'Mato Grosso'), ('MS', 'Mato Grosso do Sul'), ('MG', 'Minas Gerais'), ('PA', 'Pará'), ('PB', 'Paraíba'), ('PR', 'Paraná'), ('PE', 'Pernambuco'), ('PI', 'Piauí'), ('RJ', 'Rio de Janeiro'), ('RN', 'Rio Grande do Norte'), ('RS', 'Rio Grande do Sul'), ('RO', 'Rondônia'), ('RR', 'Roraima'), ('SC', 'Santa Catarina'), ('SP', 'São Paulo'), ('SE', 'Sergipe'), ('TO', 'Tocantins')], max_length=2)),
                ('cidade', models.CharField(max_length=300)),
                ('bairro', models.CharField(max_length=300)),
                ('logradouro', models.CharField(max_length=200)),
                ('complemento', models.CharField(max_length=300)),
                ('id_usuario', models.ForeignKey(null=True, on_delete=django.db.models.deletion.PROTECT, to=settings.AUTH_USER_MODEL)),
                ('id_fornecedor', models.ForeignKey(null=True, on_delete=django.db.models.deletion.PROTECT, to='models.fornecedor')),
            ],
        ),
        migrations.CreateModel(
            name='Emails',
            fields=[
                ('id', models.BigAutoField(primary_key=True, serialize=False)),
                ('email_primario', models.EmailField(max_length=254)),
                ('email_secundario', models.EmailField(max_length=254, null=True)),
                ('id_usuario', models.ForeignKey(null=True, on_delete=django.db.models.deletion.PROTECT, to=settings.AUTH_USER_MODEL)),
                ('id_fornecedor', models.ForeignKey(null=True, on_delete=django.db.models.deletion.PROTECT, to='models.fornecedor')),
            ],
        ),
        migrations.CreateModel(
            name='Produto',
            fields=[
                ('id', models.BigAutoField(primary_key=True, serialize=False)),
                ('nome', models.CharField(max_length=300)),
                ('preço', models.FloatField()),
                ('unidade_escolhida', models.CharField(choices=[('mg', 'Miligramas'), ('g', 'Gramas'), ('kg', 'Quilogramas'), ('ml', 'Mililitros'), ('L', 'Litros')], default='mg', max_length=3)),
                ('medida', models.IntegerField()),
                ('quantidade_total', models.PositiveIntegerField()),
                ('categoria', models.ForeignKey(on_delete=django.db.models.deletion.PROTECT, to='models.categoria')),
            ],
        ),
        migrations.AddField(
            model_name='estoque',
            name='id_Produto',
            field=models.ForeignKey(on_delete=django.db.models.deletion.PROTECT, to='models.produto'),
        ),
        migrations.CreateModel(
            name='Telefone',
            fields=[
                ('id', models.BigAutoField(primary_key=True, serialize=False)),
                ('numero_primario', models.CharField(max_length=11)),
                ('numero_secundario', models.CharField(max_length=11, null=True)),
                ('id_fornecedor', models.ForeignKey(null=True, on_delete=django.db.models.deletion.PROTECT, to='models.fornecedor')),
                ('id_usuario', models.ForeignKey(null=True, on_delete=django.db.models.deletion.PROTECT, to=settings.AUTH_USER_MODEL)),
            ],
        ),
    ]
