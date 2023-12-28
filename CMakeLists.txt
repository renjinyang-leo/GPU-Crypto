cmake_minimum_required(VERSION 3.17)
project(GPU-CRYPTO)
cmake_policy(SET CMP0104 OLD)

enable_language(CUDA)

set(CMAKE_CXX_STANDARD 11)

add_executable(gpu_aes gpu_aes.cu)