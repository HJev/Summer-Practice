
#include <iostream>
int main()
{
	int original_arr[12] = { 0, 1, 2, 2, 1, 0, 0, 2, 0, 1, 1, 0 };
	int sorted_arrt[12];
	
	int zeros = 0;
	int ones = 0;
	int twos = 0;


	for (int i = 0; i <12; i++){
		switch(original_arr[i]) {
			case 0:
				zeros += 1;
				break;
			case 1:
				ones += 1;
				break;
			case 2:
				twos += 1;
				break;

		}

	}
	for (int i = 0; i <zeros; i++){
		sorted_arrt[i] = 0;
	}
	for (int i = zeros; i <zeros + ones; i++){
		sorted_arrt[i] = 1;
	}
	for (int i = zeros + ones; i <zeros + ones + twos; i++){
		sorted_arrt[i] = 2;
	}
	std::cout << "Array Elements: ";
	for (int i = 0; i <12; i++){
		std::cout << sorted_arrt[i] << " ";
	}
	std::cout << "\n";
}
