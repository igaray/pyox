import pyox
from vm import VM

def main():
    print("p > pyox")
    print(vm)
    vm.method_m(1)
    print(pyox.sum_as_string(5, 20))
    return 0

vm = VM()

if __name__ == "__main__":
    main()
