#include <iostream>
#include "../../dependencies/infint.h"
using namespace std;

int main()
{
	InfInt x = 1;
	while(true)
	{
		cout << x*x << endl;
		x++;
	}
	return 0;
}
