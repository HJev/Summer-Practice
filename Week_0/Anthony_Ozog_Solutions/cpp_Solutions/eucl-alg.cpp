
#include <iostream>

int gcd(int num_1, int num_2){
	if (num_1 % num_2 == 0){
		return num_2;
	}
	return gcd(num_2, num_1 % num_2);
}

int main() 
{
	int num_1;
	int num_2;

	std::cin >> num_1;
	std::cin >> num_2;
	
	int gcd_num; 
	if (num_1 > num_2) {
		gcd_num = gcd(num_1, num_2);
	} else{
		gcd_num = gcd(num_2, num_1);
	}
	std::cout << gcd_num << "\n";
}

