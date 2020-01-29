#include <cstdlib>
#include <iostream>

#include "CharArrayTests.h"
#include "../MemCharArray.h"
#include "BitSequenceArrayTest.h"
#include "../BitSequenceArray.h"

using namespace std;

void memArrayTests();
void bitSeqArrayTests();

int main(int argc, char** argv) {
    memArrayTests();
    bitSeqArrayTests();
    return 0;
}

void memArrayTests() {
    cout<<"MEMORY CHAR ARRAY TESTS..."<<endl;
    CharArrayTests<MemCharArray> memArrayTests;    
    memArrayTests.basicInterfaceTest();
}

void bitSeqArrayTests() {
    cout<<"BIT SEQUENCE ARRAY TESTS..."<<endl;
    BitSequenceArrayTest<BitSequenceArray<MemCharArray> > bsaTest;
    bsaTest.basicTest();
}