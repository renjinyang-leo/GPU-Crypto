cmake_minimum_required(VERSION 3.17)

SET(CUDA_SEPARABLE_COMPILATION ON)

project(GPU-CRYPTO CUDA)
cmake_policy(SET CMP0104 OLD)

enable_language(CUDA)

set(CMAKE_CXX_STANDARD 11)

set(CMAKE_CUDA_ARCHITECTURES 86)

add_library(aes_key aes_key.cu)

add_executable(gpu_aes gpu_aes.cu)
add_executable(gpu_ore gpu_ore.cu)
add_executable(cpu_ore cpu_ore.cu)
add_executable(gpu_ore_v2 gpu_ore_v2.cu)

target_link_libraries(gpu_aes aes_key)
target_link_libraries(gpu_ore aes_key)
target_link_libraries(cpu_ore aes_key)
target_link_libraries(gpu_ore_v2 aes_key)


target_compile_options(gpu_ore PRIVATE $<$<COMPILE_LANGUAGE:CUDA>:
                       -arch=sm_61;
                       --relocatable-device-code=true
                       >)

set_property(TARGET gpu_ore 
            PROPERTY CUDA_SEPARABLE_COMPILATION ON)

target_compile_options(gpu_ore_v2 PRIVATE $<$<COMPILE_LANGUAGE:CUDA>:
        -arch=sm_61;
        --relocatable-device-code=true
        >
        -g -G)

set_property(TARGET gpu_ore_v2
        PROPERTY CUDA_SEPARABLE_COMPILATION ON)