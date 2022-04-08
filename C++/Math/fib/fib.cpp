#include <iostream>
using namespace std;

void fib()
{
	unsigned long long a = 0;
	unsigned long long b = 1;
	unsigned long long c;
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
