echo off && cls

py -m manage collectstatic

py -m manage makemigrations

py -m manage migrate

start http://127.0.0.1:8000/

py -m manage runserver