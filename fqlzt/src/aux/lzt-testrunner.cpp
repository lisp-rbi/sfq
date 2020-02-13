/** 
 * Entrypoint for running all the package tests.
 */

#include "serialization/test/serializationTestRunner.h"
#include "node_array/compact_array/test/compactArrayTestRunner.h"

using namespace std;

int main(int argc, char** argv) {
    runSerializationTests();
    //testSymbolArray();
    return 0;
}

