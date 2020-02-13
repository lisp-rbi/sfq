#ifndef BITSEQUENCEARRAYTEST_H
#define	BITSEQUENCEARRAYTEST_H

#include <sstream>
#include <iostream>

#include "debug/lzt_test.h"
#include "serialization_legacy/serialization.h"
#include "../BitSequenceArray.h"
#include "util/TempFile.h"

template <typename TBitSequenceArray>
class BitSequenceArrayTest {
public:
    BitSequenceArrayTest() {};    
    virtual ~BitSequenceArrayTest() {};

    void basicTest();
    void testAccess();
    void testResize();
    void testChangeFormat();
    void testPersistence();    

private:

    static size_t const ONE = 1;

    void readWriteNumberResize(long initSize, int B, long resizeStep, int numSteps);
    void readWriteNumbers(TBitSequenceArray* array, size_t arraySize, int numberSize);
    void serializeArrayOfRandomSeqs(size_t numOfSequences, int bitsPerSeq, bool toFolder);

};

template <typename TBitSequenceArray>
void BitSequenceArrayTest<TBitSequenceArray>::basicTest() {
    TBitSequenceArray bsa;
}

template <typename TBitSequenceArray>
void BitSequenceArrayTest<TBitSequenceArray>::testAccess() {
    cout<<"BitSequenceArrayTest.testAccess"<<endl; 
    // array with one bit seqneces
    readWriteNumbers(NULL, numberOfValues(1), 1);
    // numbers are same size as array's blocks
    readWriteNumbers(NULL, numberOfValues(BITS_PER_CHAR), BITS_PER_CHAR);
    // numbers are have one bit less than array blocks, so they span across blocks.
    readWriteNumbers(NULL, numberOfValues(BITS_PER_CHAR - 1), BITS_PER_CHAR - 1);
    // more span across blocks
    readWriteNumbers(NULL, numberOfValues(5), 5);
    readWriteNumbers(NULL, numberOfValues(13), 13);
    
    int arraySize = 10000;
    // test with bit sequences whose size is multiple of block size
    readWriteNumbers(NULL, arraySize, BITS_PER_CHAR * 2);
    readWriteNumbers(NULL, arraySize, BITS_PER_CHAR * 5);

    arraySize = 20000;
    // some large ugly bit sequences
    readWriteNumbers(NULL, arraySize, 79);
    readWriteNumbers(NULL, arraySize, 113);
    cout<<"BitSequenceArrayTest.testAccess "<<"PASSED"<<endl<<endl;
}


template <typename TBitSequenceArray>
void BitSequenceArrayTest<TBitSequenceArray>::testChangeFormat() {
    cout<<"BitSequenceArrayTest.testChangeFormat"<<endl; 
    TBitSequenceArray* array = new TBitSequenceArray();

    // test array enlargement
    for (int i = 1; i < 15; ++i) {
        array->changeFormat(numberOfValues(i), i);
        readWriteNumbers(array, numberOfValues(i), i);
    }

    // create large array
    array->changeFormat(numberOfValues(20), 20);
    // test array when size is falling
    for (int i = 15; i > 0; --i) {
        array->changeFormat(numberOfValues(i), i);
        readWriteNumbers(array, numberOfValues(i), i);
    }

    delete array;
    cout<<"BitSequenceArrayTest.testChangeFormat "<<"PASSED"<<endl<<endl; 
 }

template <typename TBitSequenceArray>
void BitSequenceArrayTest<TBitSequenceArray>::testResize() {
    cout<<"BitSequenceArrayTest.testResize"<<endl; 
    readWriteNumberResize(113, 3, 15, 10);
    readWriteNumberResize(129, 5, -11, 10);
    readWriteNumberResize(1000, 8, 100, 6);
    readWriteNumberResize(5000, 13, 1000, 3);
    readWriteNumberResize(3000, 11, -1000, 2);
    readWriteNumberResize(70417, 21, 40000, 1);
    readWriteNumberResize(60217, 23, -10000, 1);
    cout<<"BitSequenceArrayTest.testResize "<<"PASSED"<<endl<<endl; 
}

/** Fill an array, cyclically, with binary representations of all representable
 * by B bits. Resize the array filling only newly allocated positions, if any,
 * and assert the array holds the right numbers. */
template <typename TBitSequenceArray>
void BitSequenceArrayTest<TBitSequenceArray>::
readWriteNumberResize(long initSize, int B, long resizeStep, int numSteps) {
    cout<<"BitSequenceArrayTest.readWriteNumberResize("<<initSize<<","<<B<<","
            <<resizeStep<<","<<numSteps<<")"<<endl; 
    TBitSequenceArray array(initSize, B);
    // number of integers representable by B bits
    long N = 1;
    for (int i = 0; i < B; ++i) N *= 2;
    // fill array
    for (long i = 0; i < initSize; ++i) {
        long num = i % N;
        BitSequence bits = numberToBits(num);
        array.setSequence(i, bits);
    }

    ostringstream m;
    m << "initSize: " << initSize << " B: " << B << " N: " << N
            << " step: " << resizeStep << " numSteps: " << numSteps << endl;
    // test resizing
    long size = initSize;
    for (int i = 0; i < numSteps; ++i) {
        long oldSize = size;
        size += resizeStep;
        if (size <= 0) break;
        array.resize(size);
        // fill newly created space with bits
        if (size >= oldSize) {
        for (long j = oldSize; j < size; ++j) {
            long num = j % N;
            BitSequence bits = numberToBits(num);
            array.setSequence(j, bits);
        }
        }
        // check that correct number are in the array
        for (long j = 0; j < size; ++j) {
            BitSequence bits = array[j];
            long num = numberFromBits<long>(bits, B);

            ostringstream m2;
            m2 << "size: " << size << " j: " << j
                    << " expected: " << j % N << " num: " << num <<  endl;

            TEST_ASSERT_MESSAGE(num == j % N, m.str() + m2.str());
        }
    }
    cout<<"BitSequenceArrayTest.readWriteNumberResize "<<"PASSED"<<endl;
}

/** Write and read BitSequences of size numberSize interpreting them as
 * binary form of natural numbers in range [0, arraySize> .
 * Largest number representable by numberSize bits must not be less than arraySize */
template <typename TBitSequenceArray>
void BitSequenceArrayTest<TBitSequenceArray>::readWriteNumbers(TBitSequenceArray* array, size_t arraySize, int numberSize) {
    bool created = false;
    cout<<"BitSequenceArrayTest.readWriteNumbers("<<arraySize<<","<<numberSize<<")"<<endl; 
    if (array == NULL) {
        array = new TBitSequenceArray(arraySize, numberSize);
        created = true;        
    }    
    
    // create debug message header
    ostringstream ss; 
    ss << "readWriteNumbers(), arraySize: " << arraySize << " numberSize: " << numberSize << endl;
    string header = ss.str();

    // write numbers
    for (size_t i = 0; i < arraySize; ++i) {
        size_t number = i;
        BitSequence bits;
        // write number to bits
        bits.setFalse();
        for (int bi = 0; bi < numberSize; ++bi) {
            bool bit = (bool) (number & (ONE << bi));
            bits.setBit(bi, bit);
        }

        array->setSequence(i, bits);
    }

    // read numbers
    for (size_t i = 0; i < arraySize; ++i) {
        BitSequence bits = (*array)[i];
        // read number from bits
        size_t number = 0, shift = 1;
        for (int bi = 0; bi < numberSize; ++bi) {
            size_t bit = bits[bi] ? 1 : 0;
            if (bit) number += shift * bit;
            
            shift *= 2;
        }

        ostringstream ss; ss << "i: " << i << ", number: " << number;
        TEST_ASSERT_MESSAGE(number == i, header + ss.str());
        
    }

    if (created) delete array;
    cout<<"BitSequenceArrayTest.readWriteNumbers "<<"PASSED"<<endl; 
}

template <typename TBitSequenceArray>
void BitSequenceArrayTest<TBitSequenceArray>::testPersistence() {
    cout<<"BitSequenceArrayTest.testPersistence"<<endl; 
    size_t numOfSeqs = 1000;
    for (int toFolder = 0; toFolder < 2; ++toFolder) {
        serializeArrayOfRandomSeqs(numOfSeqs, 1, toFolder);
        serializeArrayOfRandomSeqs(numOfSeqs, BITS_PER_CHAR - 1, toFolder);
        serializeArrayOfRandomSeqs(numOfSeqs, BITS_PER_CHAR, toFolder);
        serializeArrayOfRandomSeqs(numOfSeqs, 13, toFolder);
        serializeArrayOfRandomSeqs(numOfSeqs, 79, toFolder);
        serializeArrayOfRandomSeqs(numOfSeqs, 113, toFolder);
    }    
    cout<<"BitSequenceArrayTest.testPersistence PASSED"<<endl<<endl; 
}


/** Generate BitSequenceArray of random sequences, serialze to file and check
 * it is equal to the read array. */
template <typename TBitSequenceArray>
void BitSequenceArrayTest<TBitSequenceArray>::serializeArrayOfRandomSeqs(size_t numOfSequences, int bitsPerSeq, bool toFolder) {
    cout<<"BitSequenceArrayTest.serializeArrayOfRandomSeqs("<<numOfSequences<<","<<bitsPerSeq<<")"
            " toFolder:"<<toFolder<<endl; 
    TBitSequenceArray array(numOfSequences, bitsPerSeq);
    for (size_t i = 0; i < numOfSequences; ++i) {
        BitSequence bits;
        for (int j = 0; j < bitsPerSeq; ++j) {
            int rand = getRandomNumber<int>(0, 100);
            bits.setBit(j, rand % 2 == 0);
        }
        array.setSequence(i, bits);
    }

    TempFile file(toFolder); 
    array.persist(file.getName());

    TBitSequenceArray* arrayDeser = new TBitSequenceArray();
    arrayDeser->load(file.getName());

    ostringstream ss;
    ss << "numOfSeq: " << array.getNumOfSequences()
       << " bitsPerSeq: " << array.getSequenceSize() << endl
       << "ser numOfSeq: " << arrayDeser->getNumOfSequences()
       << " ser bitsPerSeq: " << arrayDeser->getSequenceSize() << endl;
    // check size parameters equality
    TEST_ASSERT_MESSAGE(array.getNumOfSequences() == arrayDeser->getNumOfSequences(), ss.str());
    TEST_ASSERT_MESSAGE(array.getSequenceSize() == arrayDeser->getSequenceSize(), ss.str());

    for (size_t i = 0; i < array.getNumOfSequences(); ++i) {
        BitSequence bits = array[i];
        BitSequence bitsDeser = (*arrayDeser)[i];

        ostringstream ss;
        ss << "numOfSeq: " << array.getNumOfSequences()
           << " bitsPerSeq: " << array.getSequenceSize() << endl
           << " i: " << i << endl << "bits: " << bits.toString() << endl
           << "bits deser: " << bitsDeser.toString() << endl;

        bool bitsEqual = true;
        for (int j = 0; j < bitsPerSeq; ++j)
            if (bits[j] != bitsDeser[j]) {
                bitsEqual = false;
                break;
            }

        TEST_ASSERT_MESSAGE(bitsEqual, ss.str());
    }

    delete arrayDeser;
}


#endif	/* BITSEQUENCEARRAYTEST_H */

