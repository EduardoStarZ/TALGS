from django.contrib.auth.models import User, Group

def group_user(username):
    group = Group.objects.get(name="Client")
    user = User.objects.get(username=username)
    
    user.groups.add(group)
    