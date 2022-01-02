from random import randint, seed
from datetime import datetime


seed(datetime.utcnow())
print(datetime.utcnow())
print(randint(1, 10))
print(randint(1, 10))


class test:
    def __init__(self):
        self.cagas = randint(1, 100000)

    def set_test(self, jej):
        self.jej = jej
        return self.jej

    def return_jej(self):
        return self.jej

    def print_cagas(self):
        print(self.cagas)
        return self.cagas


t = test()

t.set_test(123)
print(t.return_jej())
t.set_test("Hello, world!")
print(t.return_jej())
t.print_cagas()

t2 = test()

t.print_cagas()
