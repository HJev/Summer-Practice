// need to understand c++

#include <iostream>

int main ()
{

	int num; 
	std::cin >> num;
	
	int new_num = 0;
	int sum = 0;

	for (int i = 0; i < num; i++){
		std::cin >> new_num;
		sum += new_num;
	}
	std::cout << sum;



	//std::cout << num;
	
}
