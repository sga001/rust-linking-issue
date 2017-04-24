#include "time.hpp"
#include <boost/chrono/chrono.hpp>

void print_time() {
  boost::chrono::nanoseconds start;
  std::cout << "time: " << start.count() << std::endl;
}
