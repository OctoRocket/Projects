#include <iostream>
#include "../dependencies/infint.h"
using namespace std;

void fib()
{
	InfInt a = 0;
	InfInt b = 1;
	InfInt c;
	while (true)
	{
		cout << b << endl;
		c = a + b;
		a = b;
		b = c;
	}
}

int main()
{
	fib();
	return 0;
}
