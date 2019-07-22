#include <iostream>
#include <string>
#include <unistd.h>

int main()
{
	std::string str;

	while (1)
	{
		getline(std::cin, str);
        usleep(1000000);
		std::cout << str << std::endl;
	}

	return (0);
}
