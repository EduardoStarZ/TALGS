::
::
:: venv.bat
::
:: Copyright (c) 2023-2024 (authors)
::
:: All rights reserved
::
:: TALGS is distributed under the () license, see LICENSE for details
::
::

@ .\color.exe "Creating a virtual environment for Windows at the T.A.L.G.S. root folder with name 'windows'" -blue -bold
@ python -m venv windows

@ .\color.exe "Installing the dependencies into the virtual environment through PIP" -yellow -bold
@ windows\Scripts\pip.exe install -r requirements.txt

@ .\color.exe "Virtual environment installed successfully" -green -bold
