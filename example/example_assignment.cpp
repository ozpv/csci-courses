#include <iostream>
#include <fstream>
#include <cstdlib>
int main(void) {
	system("ls -la");
	// one downside is a student has to specifiy
	// the dir we mounted to so ./code/
	std::ifstream in("code/read.txt");
	if (!in.is_open()) {
		std::cerr << "read.txt failed to open" << std::endl;
		return 1;
	}
	std::string line;
	while (getline(in, line)) {
		std::cout << line << std::endl;
	}
	in.close();
	std::ofstream out("write.txt");
	if (!out.is_open()) {
		std::cerr << "write.txt failed to open" << std::endl;
		return 1;
	}
	int arr[5] = { 1, 2, 3, 4, 5 };
	for (int i : arr) {
		out << i << '\n';
	}
	out.close();
	system("cat ./write.txt");
	system("ls -la");
	// try to nuke the container
	// won't matter
	system("rm -rf /tmp");
	system("ls -la /")
	return 0;
}
