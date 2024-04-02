# Django

Dependencies: 

```
pip install django
```

## Quick start 

To start project:

```
django-admin startproject mysite
```

To run server:

```
python manage.py runserver 0.0.0.0:8000
```

To create admin user:  (You need to migrate the database first)

```
python manage.py migrate
python manage.py createsuperuser
```

## Models

django use models to define the structure of database. That is, it will automatically create, manage, and generate miscenellous functions database tables by the definition of models.

You need to launch an app before defining models. 

```
python manage.py startapp polls
```
Active the app by adding the app name to `INSTALLED_APPS` in `mysite/settings.py`. 

```python 
INSTALLED_APPS = [
    'polls.apps.PollsConfig',
    'django.contrib.admin',
    'django.contrib.auth',
    'django.contrib.contenttypes',
    'django.contrib.sessions',
    'django.contrib.messages',
    'django.contrib.staticfiles',
]
```

Then, you can define models in `polls/models.py`. 

```python
from django.db import Models

class Question(models.Model):
    question_text = models.CharField(max_length=200)
    pub_date = models.DateTimeField('date published')
```

After changing the models, run

```
python manage.py makemigrations polls # poll is the name of the app and module
```

You can see the exact database transations to be run by 

```
python manage.py sqlmigrate polls 0001
```

To apply operation, run 

```
python manage.py migrate
```

For models to be visible in admin page, you need to register the model in `polls/admin.py`. 

```python
from django.contrib import django 
from .models import Question 

admin.site.register(Question)
``` 

### Deployment

Run some deployment checks:

```
manage.py check --deploy
``` 
