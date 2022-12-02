
add_executable(
  lc
  15_three_sum.cpp
  33_search.cpp
  34_search_range.cpp
  74_search_matrix.cpp
  82_delete_duplicates.cpp
  128_longest_consecutive.cpp
  129_sum_numbers.cpp
  134_gas_station.cpp
  153_search_min.cpp
  162_find_peak_element.cpp
)

target_link_libraries(
  lc
  Catch2::Catch2WithMain
)
