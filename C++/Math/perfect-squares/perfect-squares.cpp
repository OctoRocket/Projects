#include <iostream>
#include <vector>
using namespace std;

vector<int> p_squares(int limit)
{
    int c = 1;
    vector<int> p_squares;
    while (c * c <= limit)
    {
        p_squares.push_back(c * c);
        c++;
    }
    return p_squares;
}

int main()
{
    int x;
    cin >> x;
    vector<int> squares = p_squares(x);
    for (int i = 0; i <= squares.size() - 1; i++)
    {
        if (i != squares.size() - 1) {
            cout << squares[i] << ", ";
        }
        else {
            cout << squares[i] << "\n";
        }
    }
}
