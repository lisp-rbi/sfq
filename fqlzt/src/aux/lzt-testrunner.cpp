/** 
 * Entrypoint for running all the package tests.
 */

#include "serialization/test/serializationTestRunner.h"
#include "node_array/compact_array/test/compactArrayTestRunner.h"
#include "util/caching/cacheTestRunner.h"

using namespace std;

int main(int argc, char** argv) {
    runSerializationTests();
    runSymbolArrayTests();
    runCompactArrayTests();
    runCacheTests();
    return 0;
}

