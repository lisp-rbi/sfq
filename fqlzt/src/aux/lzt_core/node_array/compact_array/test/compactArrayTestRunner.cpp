#include "compactArrayTestRunner.h"

void runSymbolArrayTests() {
    memSymbArrayTests();
    diskSymbArrayTests();
}

void runCompactArrayTests() {
    typedef BitSequenceArray<DiskCharArray> TBitSequenceArray;
//    CompactArray<char, int, TBitSequenceArray> arr();
//    CompactArrayBuilder<char, int, TBitSequenceArray> builder;
    CompactArrayTester<char, int, TBitSequenceArray> tester;
    tester.testBuildSave("small-dicts");
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
