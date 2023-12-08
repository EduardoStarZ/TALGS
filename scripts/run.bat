echo off && cls

py -m manage migrate

py -m manage runserver

start http://127.0.0.1:8000/