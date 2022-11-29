
add_executable(
  lc
  34_search_range.cpp
  128_longest_consecutive.cpp
  129_sum_numbers.cpp
  134_gas_station.cpp
)

target_link_libraries(
  lc
  Catch2::Catch2WithMain
)
