#include <iostream>
using namespace std;

bool prime(unsigned long long n)
{
    if (n == 1)
    {
        return false;
    }
    for (unsigned long long i = 2; i < n; i++)
    {
        if (n % i == 0)
        {
            return false;
        }
    }
    return true;
}

int main()
{
    unsigned long long i = 1;
    while (true)
    {
        bool x = prime(i);
        if (x == true)
        {
            cout << i << " ";
        }
        i++;
    }
    return 0;
}
