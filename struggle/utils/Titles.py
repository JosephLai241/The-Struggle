#===============================================================================
#                           Run all tracking tools
#===============================================================================
from colorama import Fore, init, Style

### Automate sending reset sequences to turn off color changes at the end of 
### every print.
init(autoreset = True)

class Titles():
    """
    Methods for printing titles.
    """

    @staticmethod
    def main_title():
        print(Fore.RED + Style.BRIGHT + r"""
 __             
/\ \__          
\ \ ,_\   ____  
 \ \ \/  /',__\ 
  \ \ \_/\__, `\
   \ \__\/\____/
    \/__/\/___/ 
""")