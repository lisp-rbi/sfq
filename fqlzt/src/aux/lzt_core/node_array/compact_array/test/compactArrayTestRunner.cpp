#include "compactArrayTestRunner.h"

void runSymbolArrayTests() {
    memSymbArrayTests();
    diskSymbArrayTests();
}

void runCompactArrayTests() {    
    typedef BitSequenceArray<MemCharArray> TMemBsa;
    typedef BitSequenceArray<DiskCharArray> TDiskBsa;
//    CompactArray<char, int, TBitSequenceArray> arr();
//    CompactArrayBuilder<char, int, TBitSequenceArray> builder;
    cout<<"COMPACT ARRAY + MEMORY CHAR ARRAY TESTS..."<<endl;
    CompactArrayTester<char, int, TMemBsa> testerM;    
    testerM.testCreate("small-dicts");
    testerM.testCreate("natural-lang");
    cout<<endl;
    cout<<"COMPACT ARRAY + DISK CHAR ARRAY TESTS..."<<endl;
    CompactArrayTester<char, int, TDiskBsa> testerD;    
    testerD.testCreate("small-dicts");
    testerD.testCreate("natural-lang");
    cout<<endl;    
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
