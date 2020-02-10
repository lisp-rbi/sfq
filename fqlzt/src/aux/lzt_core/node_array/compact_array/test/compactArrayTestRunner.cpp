#include "compactArrayTestRunner.h"

void testSymbolArray() {
    cout<<"SYMBOL ARRAY + MEMORY CHAR ARRAY TESTS..."<<endl;
    testSymbolArrayCreate<BitSequenceArray<MemCharArray> >();
    cout<<endl;
    cout<<"SYMBOL ARRAY + DISK CHAR ARRAY TESTS..."<<endl;
    testSymbolArrayCreate<BitSequenceArray<DiskCharArray> >();
}

