#===============================================================================
#                       Clean KeyboardInterrupt Wrapper
#===============================================================================
from colorama import Fore, init, Style

class CleanExit():
    """
    Decorator for clean KeyboardInterrupt exits.
    """

    @staticmethod
    def cleanup(function):
        def wrapper(*args):
            try:
                item = function(*args)
                return item
            except KeyboardInterrupt:
                print(Fore.RED + Style.BRIGHT + "\n\nExiting.\n")
                quit()

        return wrapper