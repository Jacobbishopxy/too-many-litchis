
add_executable(
  lc
  33_search.cpp
  34_search_range.cpp
  74_search_matrix.cpp
  128_longest_consecutive.cpp
  129_sum_numbers.cpp
  134_gas_station.cpp
  153_search_min.cpp
)

target_link_libraries(
  lc
  Catch2::Catch2WithMain
)
