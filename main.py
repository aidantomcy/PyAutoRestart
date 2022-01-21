import os
import sys
import file_update
from colorama import init
from colorama import Fore

init(autoreset=True)

def main():
    file_name = sys.argv[1]
    if os.path.exists(file_name):
        print(f"{Fore.YELLOW}PyAutoRun")
        print(f"{Fore.GREEN}Watching for file changes...")
        os.system("python %s" % file_name)

        file_updated = file_update.file_is_updated(file_name)
        while True:
            if file_updated:
                print(f"{Fore.YELLOW}Restarting due to file changes...")
                os.system("python %s" % file_name)
            else:
                pass
    
    else:
        print(f"{Fore.RED}Please enter a valid file name.")
        sys.exit()


if __name__ == "__main__":
    try:
        main()
    except IndexError:
        print("Please enter a file name that you want to run with PyAutoRun.")
        print("Usage: python main.py <file_name>")
        print("Example: python main.py test.py")
        sys.exit()
    except KeyboardInterrupt:
        pass
