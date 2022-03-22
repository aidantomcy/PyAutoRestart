import os
import sys
import time


class Watcher(object):
    """
    A class to check if a file has been updated.
    """

    running = True
    refresh_delay_secs = 1

    # Constructor
    def __init__(self, watch_file, call_func_on_change=None, *args, **kwargs):
        self._cached_stamp = 0
        self.filename = watch_file
        self.call_func_on_change = call_func_on_change
        self.args = args
        self.kwargs = kwargs

    def look(self):
        """
        Look for changes in the file.
        """
        stamp = os.stat(self.filename).st_mtime
        if stamp != self._cached_stamp:
            self._cached_stamp = stamp
            if self.call_func_on_change is not None:
                self.call_func_on_change(*self.args, **self.kwargs)

    def watch(self):
        """
        Watch the file for changes.
        """
        while self.running:
            try:
                time.sleep(self.refresh_delay_secs)
                self.look()
            except KeyboardInterrupt:
                sys.exit()
            except FileNotFoundError:
                pass
            except:
                print(f"Unhandled error: {sys.exc_info()[0]}")
