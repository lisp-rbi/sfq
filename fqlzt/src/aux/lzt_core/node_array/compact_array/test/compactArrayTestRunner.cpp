#include "compactArrayTestRunner.h"

void runSymbolArrayTests() {
    memSymbArrayTests();
    diskSymbArrayTests();
}

void memSymbArrayTests() {
    cout<<"SYMBOL ARRAY + MEMORY CHAR ARRAY TESTS..."<<endl;
    testSymbolArrayCreate<BitSequenceArray<MemCharArray> >();
    testSymbolArraySerialize<BitSequenceArray<MemCharArray> >(false);
    testSymbolArraySerialize<BitSequenceArray<MemCharArray> >(true);
    cout<<endl;
}

void diskSymbArrayTests() {
    cout<<"SYMBOL ARRAY + DISK CHAR ARRAY TESTS..."<<endl;
    testSymbolArrayCreate<BitSequenceArray<DiskCharArray> >();
    testSymbolArraySerialize<BitSequenceArray<DiskCharArray> >(false);
    testSymbolArraySerialize<BitSequenceArray<DiskCharArray> >(true);
    cout<<endl;
}
