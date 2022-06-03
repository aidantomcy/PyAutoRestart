import os
import sys
from colorama import init
from colorama import Fore
import time

init(autoreset=True)


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


def main():
    global file_name
    file_name = sys.argv[1]
    if os.path.exists(file_name):
        print(f"{Fore.YELLOW}PyAutoRun v1.0")
        print(f"{Fore.GREEN}Watching for file changes...")
        os.system(f"python {file_name}")

        file_updated = Watcher(file_name, restart_if_updated)
        file_updated.watch()
    else:
        print(
            f"{Fore.RED}Please enter a valid file name you want to run with PyAutoRestart."
        )


def restart_if_updated():
    print(f"{Fore.YELLOW}Restarting due to file changes...")
    os.system(f"python {file_name}")


if __name__ == "__main__":
    try:
        main()
    except IndexError:
        print("Please enter a file name that you want to run with PyAutoRestart.")
        print("Usage: python main.py <file_name>")
        print("Example: python main.py test.py")
        sys.exit()
    except KeyboardInterrupt:
        sys.exit()
