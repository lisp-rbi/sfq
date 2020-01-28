#include <cstdlib>

#include "CharArrayTests.h"
#include "../MemCharArray.h"

using namespace std;

void memArrayTests();

int main(int argc, char** argv) {
    memArrayTests();
    return 0;
}

void memArrayTests() {
    CharArrayTests<MemCharArray> memArrayTests;    
    memArrayTests.test1();
}