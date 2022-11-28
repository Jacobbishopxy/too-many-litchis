
add_executable(
  lc
  128_longest_consecutive.cpp
  129_sum_numbers.cpp
)

target_link_libraries(
  lc
  Catch2::Catch2WithMain
)
