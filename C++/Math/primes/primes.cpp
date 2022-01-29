#include <iostream>
#include <vector>
using namespace std;

vector<int> prime(int limit, bool debug)
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
            if (debug) {
                cout << i << "\n";
            }
        }
    }
    return prime_list;
}


int main()
{
    bool en_debug = false;
    string debug;
    cout << "Do you want debug info? (y/n)\n";
    cin >> debug;
    if (debug == "y") {
        en_debug = true;
    }
    int inp;
    cout << "Enter limit for the prime search:\n";
    cin >> inp;
    vector<int> primes = prime(inp, en_debug);
    for (int i = 0; i < primes.size(); i++) {
        if (i != primes.size() - 1) {
            cout << primes[i] << ", ";
        }
        else {
            cout << primes[i] << "\n";
        }
    }
}
