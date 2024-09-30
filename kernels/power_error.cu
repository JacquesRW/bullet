#ifdef __HIP_PLATFORM_AMD__
#include <hip/hip_runtime.h>
#endif

constexpr size_t threadsPerBlock = static_cast<size_t>(1024);

__global__ void powerErrorKernel(
    const size_t bufferSize,
    const float* inputs,
    const float* results,
    float* output,
    const float power)
{
    const size_t i = blockIdx.x * blockDim.x + threadIdx.x;

    if (i >= bufferSize)
        return;

    const float absd = abs(inputs[i] - results[i]);
    const float error = powf(absd, power);

    atomicAdd(output, error);
}

__global__ void backpropPowerErrorKernel(
    const size_t bufferSize,
    const float* inputs,
    const float* results,
    const float* output_grad,
    float* input_grads,
    const float power)
{
    const size_t i = blockIdx.x * blockDim.x + threadIdx.x;

    if (i >= bufferSize)
        return;

    const float diff = inputs[i] - results[i];
    const float absd = abs(diff);

    const float grad = power * powf(absd, power - 1) * (*output_grad);
    input_grads[i] = diff > 0.0F ? grad : -grad;
}

extern "C" void powerError(
    const size_t bufferSize,
    const float* inputs,
    const float* results,
    float* output,
    const float power)
{
    const size_t numBlocks = (bufferSize + threadsPerBlock - 1) / threadsPerBlock;
    powerErrorKernel<<<numBlocks, threadsPerBlock>>>(bufferSize, inputs, results, output, power);
}

extern "C" void backpropPowerError(
    const size_t bufferSize,
    const float* inputs,
    const float* results,
    const float* output_grad,
    float* input_grads,
    const float power)
{
    const size_t numBlocks = (bufferSize + threadsPerBlock - 1) / threadsPerBlock;
    backpropPowerErrorKernel<<<numBlocks, threadsPerBlock>>>(bufferSize, inputs, results, output_grad, input_grads, power);
}
