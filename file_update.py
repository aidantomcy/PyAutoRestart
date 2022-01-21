import os

def file_is_updated(file_name):
    """
    Checks if the given file is updated.
    Returns True if updated, else False.
    """
    class FileUpdated(object):
        def __init__(self):
            self._cached_stamp = 0
            self.filename = file_name

        def check_file_update(self):
            stamp = os.stat(self.filename).st_mtime
            if stamp != self._cached_stamp:
                self._cached_stamp = stamp
                return True
            else:
                return False

    file_is_updated = FileUpdated()
    return file_is_updated.check_file_update()
