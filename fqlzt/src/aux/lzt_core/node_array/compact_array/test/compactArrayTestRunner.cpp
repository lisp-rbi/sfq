#include "compactArrayTestRunner.h"

void runSymbolArrayTests() {
    memSymbArrayTests();
    diskSymbArrayTests();
}

void runCompactArrayTests() {    
    typedef BitSequenceArray<MemCharArray> TMemBsa;
    typedef BitSequenceArray<DiskCharArray> TDiskBsa;
    cout<<"COMPACT ARRAY + DISK CHAR ARRAY TESTS..."<<endl;
    CompactArrayTester<char, int, TDiskBsa> testerD;        
    testerD.testCreate("small-dicts");
    //testerD.testCreate("natural-lang");
    for (int toFolder = 0; toFolder < 2; ++toFolder) {
        testerD.testSerialize("small-dicts", toFolder);
        testerD.testSerialize("natural-lang", toFolder);
    }
    cout<<endl;          
    cout<<"COMPACT ARRAY + MEMORY CHAR ARRAY TESTS..."<<endl;
    CompactArrayTester<char, int, TMemBsa> testerM;    
    testerM.testCreate("small-dicts");
    //testerM.testCreate("natural-lang");
    for (int toFolder = 0; toFolder < 2; ++toFolder) {
        testerM.testSerialize("small-dicts", toFolder);    
        testerM.testSerialize("natural-lang", toFolder);    
    }
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
