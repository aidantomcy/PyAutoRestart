import os
import sys
from file_update import Watcher
from colorama import init
from colorama import Fore

init(autoreset=True)


def main():
    global file_name
    file_name = sys.argv[1]
    if os.path.exists(file_name):
        print(f"{Fore.YELLOW}PyAutoRun v1.0")
        print(f"{Fore.GREEN}Watching for file changes...")
        os.system("python %s" % file_name)

        file_updated = Watcher(file_name, restart_if_updated)
        file_updated.watch()
    else:
        print(
            f"{Fore.RED}Please enter a valid file name you want to run with PyAutoRun."
        )


def restart_if_updated():
    print(f"{Fore.YELLOW}Restarting due to file changes...")
    os.system("python %s" % file_name)


if __name__ == "__main__":
    try:
        main()
    except IndexError:
        print("Please enter a file name that you want to run with PyAutoRun.")
        print("Usage: python main.py <file_name>")
        print("Example: python main.py test.py")
        sys.exit()
    except KeyboardInterrupt:
        sys.exit()
