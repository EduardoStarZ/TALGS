@ .\color.exe "Checking for changes in the database modeling structure" -blue -italic
@ windows\Scripts\python.exe -m manage makemigrations
@ windows\Scripts\python.exe -m manage makemigrations models 

@ .\color.exe "Applying found changes into the database" -yellow -bold
@ windows\Scripts\python.exe -m manage migrate

@ .\color.exe "Injecting the static files into cache" -yellow -bold
@ windows\Scripts\python.exe -m manage collectstatic

@ .\color.exe "Migrations and static file injections done" -green -bold