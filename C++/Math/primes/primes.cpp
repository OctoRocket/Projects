#include <iostream>
#include <vector>
using namespace std;

vector<int> prime(int limit)
{
    bool found;
    vector<int> prime_list = {2};
    for (int i = 2; i < limit+1; i++) {
        for (int j = 0; j < prime_list.size()/2 + 1; j++) {
            if (i % prime_list[j] == 0 or i == prime_list[j]) {
                found = false;
                break;
            }
            else {
                found = true;
            }
        }
        if (found) {
            prime_list.push_back(i);
        }
    }
    return prime_list;
}


int main()
{
    int inp = 1000000;
    // cin >> inp;
    vector<int> primes = prime(inp);
    for (int i = 0; i < primes.size(); i++) {
        if (i != primes.size() - 1) {
            cout << primes[i] << ", ";
        }
        else {
            cout << primes[i] << "\n";
        }
    }
}
