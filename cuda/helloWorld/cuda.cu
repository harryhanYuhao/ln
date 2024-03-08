#include <iostream>
#include <stdio.h>
#include <math.h>

#define CUDA_TREADS_CU 1
// Kernel function to add the elements of two arrays
	__global__
void add(int n, float *x, float *y)
{
	for (int i = threadIdx.x; i < n; i += CUDA_TREADS_CU)
		y[i] = x[i] + y[i];
}

int main(void)
{
	int N = 1<<20;
	float *x = (float*) malloc(N*sizeof(float));
	float *y = (float*) malloc(N*sizeof(float));
	 
	float *dx, *dy;
	// Allocate Unified Memory – accessible from CPU or GPU
	cudaMalloc(&dx, N*sizeof(float));
	cudaMalloc(&dy, N*sizeof(float));
	//
	// initialize x and y arrays on the host
	for (int i = 0; i < N; i++) {
		x[i] = 1.0f;
		y[i] = (float)i;
	}
	cudaMemcpy(dx, x, N*sizeof(float), cudaMemcpyHostToDevice);
	cudaMemcpy(dy, y, N*sizeof(float), cudaMemcpyHostToDevice);
	// Run kernel on 1M elements on the GPU
	add<<<1, CUDA_TREADS_CU>>>(N, dx, dy);

	// Wait for GPU to finish before accessing on host
	cudaMemcpy(x, dx, N*sizeof(float), cudaMemcpyDeviceToHost);
	cudaMemcpy(y, dy, N*sizeof(float), cudaMemcpyDeviceToHost);

	// Check for errors (all values should be 3.0f)

	std::cout<< y[N-1] << std::endl;


	// Free memory
	cudaFree(dx);
	cudaFree(dy);
	free(x);
	free(y);

	printf("aaa\n");
	return 0;
}
