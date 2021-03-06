#include "serializationTestRunner.h"

int runSerializationTests() {
    memArrayTests();
    diskArrayTests();
    bitSeqArrayMemTests();
    bitSeqArrayDiskTests();
    return 0;
}

void memArrayTests() {
    cout<<"MEMORY CHAR ARRAY BASIC TESTS..."<<endl;
    CharArrayTests<MemCharArray> memArrayTests;    
    memArrayTests.basicInterfaceTest();
}

void diskArrayTests() {
    cout<<"DISK CHAR ARRAY BASIC TESTS..."<<endl;
    CharArrayTests<DiskCharArray> diskArrayTests;    
    diskArrayTests.basicInterfaceTest();
}

void bitSeqArrayMemTests() {
    cout<<"BIT SEQUENCE ARRAY (MEMORY) TESTS..."<<endl;
    BitSequenceArrayTest<BitSequenceArray<MemCharArray> > bsaTest;
    bsaTest.basicTest();
    bsaTest.testAccess();
    bsaTest.testChangeFormat();
    bsaTest.testResize();
    bsaTest.testPersistence(0);
    bsaTest.testPersistence(1);
}

void bitSeqArrayDiskTests() {
    cout<<"BIT SEQUENCE ARRAY (DISK) TESTS..."<<endl;
    BitSequenceArrayTest<BitSequenceArray<DiskCharArray> > bsaTest;
    bsaTest.basicTest();
    bsaTest.testAccess();
    bsaTest.testChangeFormat();
    bsaTest.testResize();
    bsaTest.testPersistence(0);
    bsaTest.testPersistence(1);   
    testDiskInPlacePersistence<BitSequenceArray<DiskCharArray> >();
}