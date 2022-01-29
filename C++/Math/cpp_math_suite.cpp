#include <iostream>
using namespace std;

int three_x_plus_one(int num)
{
    int count = 0;
    while (num > 1)
    {
        if (num % 2 == 0)
        {
            num = num / 2;
        }
        else
        {
            num = 3 * num + 1;
        }
        count++;
    }
    return count;
}

int main()
{
    bool use = true;
    while (use) {
        int inp;
        cout << "1 is a 3x+1 interface.\n";
        cin >> inp;
        if (inp == 1)
        {
            int num;
            cin >> num;
            cout << three_x_plus_one(num) << "\n";
        }
        cout << "Do you want to use this program again?\n";
    }
}
