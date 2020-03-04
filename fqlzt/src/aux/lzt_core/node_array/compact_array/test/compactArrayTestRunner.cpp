#include "compactArrayTestRunner.h"

void runSymbolArrayTests() {
    memSymbArrayTests();
    diskSymbArrayTests();
}

void runCompactArrayTests() {    
    typedef BitSequenceArray<MemCharArray> TMemBsa;
    typedef BitSequenceArray<DiskCharArray> TDiskBsa;
    //char const * dictsets[] = { "small-dicts", "natural-lang" }; int numDsets = 2;
    char const * dictsets[] = { "small-dicts", "fasta" }; int numDsets = 2;
    cout<<"COMPACT ARRAY + DISK CHAR ARRAY TESTS..."<<endl;
    CompactArrayTester<char, int, TDiskBsa> testerD;        
    for (int i = 0; i < numDsets; ++i) {
        string dictset = dictsets[i];
        testerD.testCreate(dictset);    
        { // test caching
            int cacheSize;
            if (dictset == "fasta" or dictset == "natural-lang") cacheSize = 200;
            else cacheSize = 5;
            testerD.testCaching(dictset, cacheSize);
        }
        for (int toFolder = 0; toFolder < 2; ++toFolder)
            testerD.testSerialize(dictset, toFolder);            
        testerD.testSerializeInPlace(dictset);        
    }
    cout<<endl;          
    cout<<"COMPACT ARRAY + MEMORY CHAR ARRAY TESTS..."<<endl;
    CompactArrayTester<char, int, TMemBsa> testerM;    
    for (int i = 0; i < numDsets; ++i) {
        string dictset = dictsets[i];
        testerM.testCreate(dictset);  
        { // test caching
            int cacheSize;
            if (dictset == "fasta" or dictset == "natural-lang") cacheSize = 200;
            else cacheSize = 5;
            testerM.testCaching(dictset, cacheSize);
        }        
        for (int toFolder = 0; toFolder < 2; ++toFolder)
            testerM.testSerialize(dictset, toFolder);                
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
