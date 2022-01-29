#include <iostream>
#include <cmath>
using namespace std;

double* quadratic(double a, double b, double c) {
  double x=(-b+sqrt(pow(b, 2)-4*a*c))/2*a;
  double y=(-b-sqrt(pow(b, 2)-4*a*c))/2*a;
  static double arr[2] = {x, y};
  return arr;
}

int main() {
  cout << "Quadric coefficients:" << "\n";
  double a; double b; double c;
  cin >> a; cin >> b; cin >> c;
  cout << "\n";
  double* ptr = quadratic(a, b, c);
  cout << "Answers are:\n" << ptr[0] << "\n" << ptr[1] << "\n";
}
