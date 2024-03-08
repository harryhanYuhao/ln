#include <iostream>

namespace Parser {
	void prim(bool);
	double term(bool);
}

void Parser::prim(bool term) {
	if (term) {
		std::cout<<"term is true!"<<std::endl;
	} else {
		std::cout<<"term is false!"<<std::endl;
	}
}

int main () {
	std::cout<<"Hi"<<std::endl;
	Parser::prim(false);
	return 0;
}
