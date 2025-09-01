#include "parsing/input.hpp"

std::string load_file(const std::filesystem::path &path) {
  std::string contents(std::filesystem::file_size(path), '\0');
  std::ifstream(path, std::ios::binary).read(contents.data(), contents.size());
  return contents;
}
