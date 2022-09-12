# PyMon

This is a nodemon clone but for Python.  
The main.py file performs actions if a file has been updated.  
The file_update.py has a class called Watcher with functions to watch
for file updates.  
I will soon publish this to PyPI.

## How to use:

To use this with a file, you need to run:

```
python main.py file_to_watch.py
```

If you don't provide a file, it will print this message:

```
Please enter a file name that you want to run with PyAutoRestart.
Usage: python main.py <file_name>
Example: python main.py test.py
```

Otherwise, if you pass a file that doesn't exist, it will give you this error:

```
Please enter a valid file name you want to run with PyAutoRestart.
```

If there is any other project that resembles this or is similar to this, it is a pure coincidence.
