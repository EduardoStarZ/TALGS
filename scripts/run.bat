::
::
:: run.bat
::
:: Copyright (c) 2023-2024 (authors)
::
:: All rights reserved
::
:: TALGS is distributed under the () license, see LICENSE for details
::
::

@ .\color.exe "Starting Django server protocol" -blue -bold 
@ start windows\Scripts\python.exe -m manage runserver
@ .\color.exe "Django server initialized" -green -bold

@ .\color.exe "Opening browser windows at http://127.0.0.1:8000" -blue -bold -underline
@ start http://127.0.0.1:8000/
