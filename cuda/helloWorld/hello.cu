#include <stdio.h>
#include <iostream>

__global__ void cuda_hello(){
    printf("Hello World from GPU!\n");
}

int main() {
    cuda_hello<<<1,1>>>(); 
	std::cout<<"Hi!"<<std::endl;
    return 0;
}

