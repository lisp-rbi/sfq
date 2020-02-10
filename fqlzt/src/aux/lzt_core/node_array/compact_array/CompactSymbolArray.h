#ifndef COMPACTSYMBOLARRAY_H
#define	COMPACTSYMBOLARRAY_H

#include <cstddef>
#include <map>
#include <cassert>

using namespace __gnu_cxx;
using namespace std;

/** All the different symbols are stored in a table and the array
 * is stored an BitSequenceArray of indexes. Efficient for storing 
 * large array with small number of distinct symbols. */
template <typename TSymbol, typename TBitSequenceArray>
class CompactSymbolArray {
public:

    CompactSymbolArray();
    CompactSymbolArray(TSymbol const * symbols, size_t numOfSymbols);
    virtual ~CompactSymbolArray();

    TSymbol operator[](size_t i) const;
    size_t size() const;
    void createFromArray(TSymbol const * symbols, size_t numOfSymbols);

    //template <typename TSymb> friend class CompactSymbolArraySerL;

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

};

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

#endif	/* COMPACTSYMBOLARRAY_H */

