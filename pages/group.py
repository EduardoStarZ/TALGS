from django.contrib.auth.models import User, Group

def get_user_group(username):
    user = User.objects.get(username=username)
    
    groups = user.groups.all()
    
    if len(groups) == 0:
        return "Administrator"    
    print(groups[0])
    return groups[0].__str__