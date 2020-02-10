#ifndef COMPACTARRAYTESTRUNNER_H
#define COMPACTARRAYTESTRUNNER_H

#include "CompactSymbolTester.h"
#include "serialization/MemCharArray.h"
#include "serialization/DiskCharArray.h"
#include "serialization/BitSequenceArray.h"

void testSymbolArray();

template<typename TBitSequenceArray>
void testSymbolArrayCreate() {
    //TODO ove konstante za minimume i maksimume bi trebale biti posebno definirane    
    {
        CompactSymbolTester<char, TBitSequenceArray> tester(-128, 127, 300, false);
        tester.testCreate();
    }
    {
        CompactSymbolTester<char, TBitSequenceArray> tester(-128, 127, 200000, false);
        tester.testCreate();
    }
    {
        CompactSymbolTester<short, TBitSequenceArray> tester(-20000, -19900, 1100, true);
        tester.testCreate();
    }
    {
        CompactSymbolTester<unsigned int, TBitSequenceArray> tester(199999900, 200000000, 2100, true);
        tester.testCreate();
    }
    cout<<"SYMBOL ARRAY TEST CREATE PASSED"<<endl;
}

#endif /* COMPACTARRAYTESTRUNNER_H */

