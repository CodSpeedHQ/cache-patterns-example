cmake_minimum_required(VERSION 3.12)
project(cache_patterns VERSION 1.0.0 LANGUAGES CXX)

# Set C++ standard
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Use RelWithDebInfo for debug symbols (important for CodSpeed profiling)
if(NOT CMAKE_BUILD_TYPE)
    set(CMAKE_BUILD_TYPE RelWithDebInfo)
endif()

# Include FetchContent for downloading dependencies
include(FetchContent)

# Enable downloading dependencies for benchmark
set(BENCHMARK_DOWNLOAD_DEPENDENCIES ON)

# Disable specific warnings for CodSpeed/Google Benchmark
if(CMAKE_CXX_COMPILER_ID MATCHES "Clang" OR CMAKE_CXX_COMPILER_ID MATCHES "AppleClang")
    add_compile_options(-Wno-sign-conversion -Wno-error)
elseif(CMAKE_CXX_COMPILER_ID MATCHES "GNU")
    add_compile_options(-Wno-sign-conversion -Wno-error)
endif()

# Fetch CodSpeed's Google Benchmark compatibility layer
FetchContent_Declare(
    google_benchmark
    GIT_REPOSITORY https://github.com/CodSpeedHQ/codspeed-cpp
    GIT_TAG main
    SOURCE_SUBDIR google_benchmark
)

FetchContent_MakeAvailable(google_benchmark)

# Create library for particle simulation code
add_library(cache_patterns_lib
    src/aos.cpp
    src/soa.cpp
)

target_include_directories(cache_patterns_lib PUBLIC
    ${CMAKE_CURRENT_SOURCE_DIR}/include
)

# Create benchmark executable
add_executable(particle_simulation_bench
    benchmarks/particle_simulation.cpp
)

target_link_libraries(particle_simulation_bench
    cache_patterns_lib
    benchmark::benchmark
)

target_include_directories(particle_simulation_bench PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/include
)
