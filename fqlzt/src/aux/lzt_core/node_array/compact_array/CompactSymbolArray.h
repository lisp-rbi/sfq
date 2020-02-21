#ifndef COMPACTSYMBOLARRAY_H
#define	COMPACTSYMBOLARRAY_H

#include <cstddef>
#include <map>
#include <cassert>
#include <iostream>
#include <fstream>

#include "serialization_legacy/BitSequence.h"
#include "serialization/BitSequenceArray.h"
#include "serialization/MemCharArray.h"
#include "serialization/ISerializable.h"
#include "serialization_legacy/serialization.h"
#include "serialization_legacy/SerializationUtils.h"
#include "util/filesystem_utils.h"
#include "util/constants.h"

using namespace __gnu_cxx;
using namespace std;

/** All the different symbols are stored in a table and the array
 * is stored an BitSequenceArray of indexes. Efficient for storing 
 * large array with small number of distinct symbols. */
template <typename TSymbol, typename TBitSequenceArray>
class CompactSymbolArray : public ISerializable {
public:

    CompactSymbolArray();
    CompactSymbolArray(TSymbol const * symbols, size_t numOfSymbols);
    virtual ~CompactSymbolArray();

    TSymbol operator[](size_t i) const;
    size_t size() const;
    void createFromArray(TSymbol const * symbols, size_t numOfSymbols);

    //template <typename TSymb> friend class CompactSymbolArraySerL;
    
    bool persist(string f);
    bool load(string f);
    void writeToStream(ostream& stream);
    void readFromStream(istream& stream);
    
    template <typename TS, typename TI, typename TBsa> friend class CompactArrayBuilder;

private:

    // array size
    size_t numOfSymbols;
    // number of distinct symbols
    size_t numOfDistinct;
    // table of distinct symbols
    TSymbol* symbolTable;

    TBitSequenceArray indexes;    
    // number of bits necessary to store an index
    int bitsPerIndex;

    void freeTable();
    
    void writeFieldsToStream(ostream& stream);
    void readFieldsFromStream(istream& stream);
    void writeSymbolsToStream(ostream& stream);
    void readSymbolsFromStream(istream& stream);
    
    static int const BITS_PER_SYMBOL = sizeof(TSymbol) * BITS_PER_CHAR;
    static const string PERSIST_FNAME;
    static const string PERSIST_FIELDS_FNAME;
    static const string PERSIST_SYMBTABLE_FNAME;    

};

template <typename TSymbol, typename TBitSequenceArray>
const string CompactSymbolArray<TSymbol, TBitSequenceArray>::PERSIST_FNAME = "CompactSymbolArray.bin";

template <typename TSymbol, typename TBitSequenceArray>
const string CompactSymbolArray<TSymbol, TBitSequenceArray>::PERSIST_FIELDS_FNAME = "CompactSymbolArrayFields.bin";

template <typename TSymbol, typename TBitSequenceArray>
const string CompactSymbolArray<TSymbol, TBitSequenceArray>::PERSIST_SYMBTABLE_FNAME = "CompactSymbolArraySymbols.bin";

template <typename TSymbol, typename TBitSequenceArray>
CompactSymbolArray<TSymbol, TBitSequenceArray>::CompactSymbolArray()
: numOfDistinct(0), numOfSymbols(0), symbolTable(NULL) {}

template <typename TSymbol, typename TBitSequenceArray>
CompactSymbolArray<TSymbol, TBitSequenceArray>::CompactSymbolArray(TSymbol const * symbols, size_t numOfSymbols)
: symbolTable(NULL) {
    createFromArray(symbols, numOfSymbols);
}

template <typename TSymbol, typename TBitSequenceArray>
CompactSymbolArray<TSymbol, TBitSequenceArray>::~CompactSymbolArray() {
    freeTable();
}

template <typename TSymbol, typename TBitSequenceArray>
void CompactSymbolArray<TSymbol, TBitSequenceArray>::freeTable() {
    if (symbolTable != NULL) delete [] symbolTable;
}

template <typename TSymbol, typename TBitSequenceArray>
inline TSymbol CompactSymbolArray<TSymbol, TBitSequenceArray>::operator[](size_t i) const {
    BitSequence indexBits = indexes[i];
    size_t tableIndex = numberFromBits<size_t>(indexBits, bitsPerIndex);
    return symbolTable[tableIndex];
}

template <typename TSymbol, typename TBitSequenceArray>
size_t CompactSymbolArray<TSymbol, TBitSequenceArray>::size() const {
    return numOfSymbols;
}

/** Set compact array to be a copy of the symbols array. */
template <typename TSymbol, typename TBitSequenceArray>
void CompactSymbolArray<TSymbol, TBitSequenceArray>::createFromArray(TSymbol const * symbols, size_t numSymbols) {
    freeTable();

    numOfSymbols = numSymbols;
   
    map<TSymbol, size_t> symbolToIndex;
    typedef typename map<TSymbol, size_t>::iterator TMapIter;


    // insert symbols in the map, only the distinct symbols are stored
    for (size_t i = 0; i < numOfSymbols; ++i)
        symbolToIndex[symbols[i]];

    numOfDistinct = symbolToIndex.size();
    symbolTable = new TSymbol[numOfDistinct];

    // store distinct symbols in the table, map them to their table indexes
    TMapIter it = symbolToIndex.begin();
    for (size_t i = 0; it != symbolToIndex.end(); ++it, ++i) {
        // store
        symbolTable[i] = (*it).first;
        // map
        (*it).second = i;
    }    
    // number of bits needed to store indexes in distinct symbol table
    bitsPerIndex = numberOfBits(numOfDistinct);
    //cout<<"bpi: "<<bitsPerIndex<<" numDist: "<<numOfDistinct<<endl;
    // reserve space for array of indexes
    indexes.changeFormat(numOfSymbols, bitsPerIndex);

    // get mapped indexes of symbols and store in the array
    for (size_t i = 0; i < numOfSymbols; ++i) {
        size_t tableIndex = symbolToIndex[symbols[i]];
        indexes.setSequence(i, numberToBits(tableIndex));
        assert(numberToBits(tableIndex).maxNonzeroBit() < bitsPerIndex );
    }
    
}

template <typename TSymbol, typename TBitSequenceArray>
bool CompactSymbolArray<TSymbol, TBitSequenceArray>::persist(string f) {
    if (file_accessible(f)) {
        if (file_is_regular(f)) {
            ofstream output(f.c_str());
            writeToStream(output);
            output.close();    
            return output.good();
        }
        else if (file_is_directory(f)) {                
            // persist fields
            string fname = accessible_filename(f, PERSIST_FIELDS_FNAME);
            if (fname == "") return false;
            ofstream fieldsstr(fname.c_str());
            writeFieldsToStream(fieldsstr); fieldsstr.close();            
            if (!fieldsstr.good()) return false;
            // persist symbol table
            fname = accessible_filename(f, PERSIST_SYMBTABLE_FNAME);
            if (fname == "") return false;
            ofstream symbstr(fname.c_str());
            writeSymbolsToStream(symbstr); symbstr.close();   
            if (!symbstr.good()) return false;
            // persist bit seq. array with indexes
            return indexes.persist(f);
        }
        else return false;
    }
    else return false;
}

template <typename TSymbol, typename TBitSequenceArray>
bool CompactSymbolArray<TSymbol, TBitSequenceArray>::load(string f) {
    if (file_accessible(f)) {
        if (file_is_regular(f)) {            
            ifstream stream(f.c_str());
            if (stream.good()) {  
                readFromStream(stream);
                stream.close();
                return stream.good();
            }
            else return false;    
        }
        else if (file_is_directory(f)) {          
            // load fields            
            string fname = accessible_filename(f, PERSIST_FIELDS_FNAME);
            if (fname == "") return false;
            ifstream fieldsstr(fname.c_str());
            readFieldsFromStream(fieldsstr); fieldsstr.close();
            if (!fieldsstr.good()) return false;
            // load symbol table            
            fname = accessible_filename(f, PERSIST_SYMBTABLE_FNAME);
            if (fname == "") return false;
            ifstream symbstr(fname.c_str());
            readSymbolsFromStream(symbstr); symbstr.close();   
            if (!symbstr.good()) return false;
            // load bit seq. array with indexes
            return indexes.load(f);
        }
        else return false;
    }
    else return false;    
}

template <typename TSymbol, typename TBitSequenceArray>
void CompactSymbolArray<TSymbol, TBitSequenceArray>::writeToStream(ostream& stream) {
    writeFieldsToStream(stream);
    writeSymbolsToStream(stream);
    indexes.writeToStream(stream);
}

template <typename TSymbol, typename TBitSequenceArray>
void CompactSymbolArray<TSymbol, TBitSequenceArray>::readFromStream(istream& stream) {
    readFieldsFromStream(stream);
    readSymbolsFromStream(stream);
    indexes.readFromStream(stream);
}

template <typename TSymbol, typename TBitSequenceArray>
void CompactSymbolArray<TSymbol, TBitSequenceArray>::writeFieldsToStream(ostream& stream) {
    SerializationUtils::integerToStream(numOfSymbols, stream);
    SerializationUtils::integerToStream(numOfDistinct, stream);
    SerializationUtils::integerToStream(bitsPerIndex, stream);
}

template <typename TSymbol, typename TBitSequenceArray>
void CompactSymbolArray<TSymbol, TBitSequenceArray>::readFieldsFromStream(istream& stream) {
    numOfSymbols = SerializationUtils::integerFromStream<size_t>(stream);
    numOfDistinct = SerializationUtils::integerFromStream<size_t>(stream);
    bitsPerIndex = SerializationUtils::integerFromStream<int>(stream);
}

template <typename TSymbol, typename TBitSequenceArray>
void CompactSymbolArray<TSymbol, TBitSequenceArray>::writeSymbolsToStream(ostream& stream) {
    BitSequenceArray<MemCharArray> symbArr(numOfDistinct, BITS_PER_SYMBOL);
    for (size_t i = 0; i < numOfDistinct; ++i)
        symbArr.setSequence(i, toBitSequence(symbolTable[i], BITS_PER_SYMBOL));
    symbArr.writeToStream(stream);
}

template <typename TSymbol, typename TBitSequenceArray>
void CompactSymbolArray<TSymbol, TBitSequenceArray>::readSymbolsFromStream(istream& stream) {
    BitSequenceArray<MemCharArray> bitArray;
    bitArray.readFromStream(stream);
    freeTable();
    symbolTable = new TSymbol[numOfDistinct];
    assert(numOfDistinct == bitArray.getNumOfSequences());
    for (size_t i = 0; i < numOfDistinct; ++i) {
        BitSequence bits = bitArray[i];
        symbolTable[i] = fromBitSequence<TSymbol>(bits, BITS_PER_SYMBOL);
    }    
}

#endif	/* COMPACTSYMBOLARRAY_H */

