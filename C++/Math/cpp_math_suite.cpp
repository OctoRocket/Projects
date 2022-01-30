#include <iostream>
using namespace std;

int three_x_plus_one(int num)
{
    int count = 0;
    while (num > 1)
    {
        if (num % 2 == 0) {
            num = num / 2;
        }
        else {
            num = 3 * num + 1;
        }
        count++;
    }
    return count;
}

void fib(int limit) {
    unsigned long long a = 0;
    unsigned long long b = 1;
    unsigned long long c;
    for (int i = 0; i < limit; i++) {
        cout << b << "\n";
        c = a + b;
        a = b;
        b = c;
    }
}

int main()
{
    bool use = true;
    while (use) {
        int inp;
        cout << "1 is for 3x+1, 2 is for fib.\n";
        cin >> inp;
        if (inp == 1) {
            cout << "Enter a number to see how long it takes to reach 1.\n";
            int num;
            cin >> num;
            cout << three_x_plus_one(num) << "\n";
        }
        else if (inp == 2) {
            cout << "Enter how many numbers of the fibonacci sequence you want to print out.\n";
            int num;
            cin >> num;
            fib(num);
        }
        string use_deter;
        cout << "Do you want to use this program again? (y/n)\n";
        cin >> use_deter;
        if (use_deter != "y") {
            break;
        }
    }
}
