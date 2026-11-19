// A C++ library with a static initializer that calls into libstdc++,
// like RocksDB's option type maps.
#include <string>
#include <unordered_map>

static const std::unordered_map<std::string, int> table = {
    {"one", 1},
    {"two", 2},
    {"three", 3},
};

extern "C" int dummy_lookup(const char *key) {
    auto it = table.find(key);
    return it == table.end() ? -1 : it->second;
}
