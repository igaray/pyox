class VM:
    def __init__(self):
        self.ip = 0
        self.code = []
        self.memory = []

    def method_m(self, n):
        print(f"p > method_m {n}")

def function_f():
    print("p > function_f")
