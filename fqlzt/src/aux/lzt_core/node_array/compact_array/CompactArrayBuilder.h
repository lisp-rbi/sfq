#ifndef COMPACTARRAYBUILDER_H
#define	COMPACTARRAYBUILDER_H

#include <cstddef>
#include <cassert>
#include <string>
#include <iostream>
#include <fstream>

#include "CompactArray.h"
#include "CompactSymbolArray.h"

#include "util/factory.h"
#include "dictionary/util/WordList.h"
#include "node_array/vector_array/VectorArray.h"
#include "node_array/compact_array_legacy/CompactArray.h"
#include "node_array/compact_array_legacy/CompactSymbolArray.h"
#include "node_array/compact_array_legacy/CompactArrayCreator.h"
#include "node_array/compact_array_legacy/utils.h"
#include "serialization_legacy/BitSequence.h"
#include "serialization_legacy/serialization.h"
#include "serialization_legacy/SerializationUtils.h"
#include "util/utils.h"

using namespace std;

template <typename TSymbol, typename TIndex, typename TBitSequenceArray>
class CompactArrayBuilder {
    
public:

    //typedef typename TNodeArray::Symbol TSymbol;
    //typedef typename TNodeArray::Index TIndex;

    //CompactArrayBuilder(TNodeArray const & nodeArray);    

    //CompactArrayL<TSymbol, TIndex>* createCompactArray();
    
    void buildSaveCompactArray(WordList<TSymbol>* words, string folder, 
                                string dictLabel, bool enumerated=false);
    
    CompactArray<TSymbol, TIndex, TBitSequenceArray> *
    createCompactArray(WordList<TSymbol>* words, string dictLabel, bool enumerated=false);

private:

    typedef VectorArray<TSymbol, TIndex> TMemNodeArray;
    
    void copyFields(CompactArray<TSymbol, TIndex, TBitSequenceArray>& ca, CompactArrayL<TSymbol, TIndex>& cal);
    
    template<typename TBsa1, typename TBsa2>
    void copyBitSeqArray(TBsa1& bsa1, TBsa2& bsa2, bool fieldsOnly);
    
    //template<typename TSa1, typename TSa2>
    //void copySymbolArray(TSa1& bsa1, TSa2& bsa2);
    void copySymbolArray(CompactSymbolArray<TSymbol, TBitSequenceArray> &sa1, 
                         CompactSymbolArrayL<TSymbol> &sa2);

};

template <typename TSymbol, typename TIndex, typename TBitSequenceArray>
void CompactArrayBuilder<TSymbol, TIndex, TBitSequenceArray>::
buildSaveCompactArray(WordList<TSymbol>* words, string folder, string dictLabel, bool enumerated) {    
    cout<<"buildSaveCompactArray(dict="<<dictLabel<<",enum="<<enumerated<<")"<<endl;
    TMemNodeArray* array = getLzArrayLCT<TMemNodeArray>(*words);
    CompactArrayCreatorL<TMemNodeArray> creator(*array);
    CompactArrayL<TSymbol, TIndex>* carrayLegacy = creator.createCompactArray();    
    delete array;
    CompactArray<TSymbol, TIndex, TBitSequenceArray> carray;    
    copyFields(carray, *carrayLegacy);
    copyBitSeqArray(carray.array, carrayLegacy->array, false);
    // init and copy bit.seq.arrays:
    // .. init with disk.array in folder/subfolder
    // .. copy BSA: copy fields, copy array data
    // copy symbol array: fields and bit.seq.array
    // save carray 2 folder
    delete carrayLegacy;
}

template <typename TSymbol, typename TIndex, typename TBitSequenceArray>
CompactArray<TSymbol, TIndex, TBitSequenceArray>* CompactArrayBuilder<TSymbol, TIndex, TBitSequenceArray>::
createCompactArray(WordList<TSymbol>* words, string dictLabel, bool enumerated=false) {
    cout<<"createCompactArray(dict="<<dictLabel<<",enum="<<enumerated<<")"<<endl;
    TMemNodeArray* array = getLzArrayLCT<TMemNodeArray>(*words);
    CompactArrayCreatorL<TMemNodeArray> creator(*array);
    CompactArrayL<TSymbol, TIndex>* carrayLegacy = creator.createCompactArray();    
    delete array;
    CompactArray<TSymbol, TIndex, TBitSequenceArray> *carray = 
            new CompactArray<TSymbol, TIndex, TBitSequenceArray>();    
    copyFields(*carray, *carrayLegacy);
    copyBitSeqArray(carray->array, carrayLegacy->array, false);
    copyBitSeqArray(carray->siblings, carrayLegacy->siblings, false);
    copyBitSeqArray(carray->numOfWords, carrayLegacy->numOfWords, false);
    copySymbolArray(carray->symbols, carrayLegacy->symbols);
    delete carrayLegacy;    
    return carray;
}

template <typename TSymbol, typename TIndex, typename TBitSequenceArray>
void CompactArrayBuilder<TSymbol, TIndex, TBitSequenceArray>::
copyFields(CompactArray<TSymbol, TIndex, TBitSequenceArray>& ca, CompactArrayL<TSymbol, TIndex>& cal) {    
    ca.numOfDistinct = cal.numOfDistinct;
    ca.numOfNodes = cal.numOfNodes;
    ca.bitsPerIndex = cal.bitsPerIndex;
    ca.enumerated = cal.enumerated;
    assert(ca.NUM_OFFSETS == cal.NUM_OFFSETS);
    for (int i = 0; i < ca.NUM_OFFSETS; ++i) 
        ca.flagOffsets[i] = cal.flagOffsets[i];
}

template <typename TSymbol, typename TIndex, typename TBitSequenceArray>
template<typename TBsa1, typename TBsa2> void CompactArrayBuilder<TSymbol, TIndex, TBitSequenceArray>::    
copyBitSeqArray(TBsa1& bsa1, TBsa2& bsa2, bool fieldsOnly) {
    if (fieldsOnly) {
        bsa1.numOfBlocks = bsa2.numOfBlocks;
        bsa1.numOfSequences = bsa2.numOfSequences;
        bsa1.bitsPerSequence = bsa2.bitsPerSequence;
    }
    else {
        bsa1.changeFormat(bsa2.getNumOfSequences(), bsa2.getSequenceSize());
        for (size_t i = 0; i < bsa2.getNumOfSequences(); ++i) 
            bsa1.setSequence(i, bsa2[i]);        
    }
}

//template <typename TSymbol, typename TIndex, typename TBitSequenceArray>    
//template<typename TSa1, typename TSa2> void CompactArrayBuilder<TSymbol, TIndex, TBitSequenceArray>::
//copySymbolArray(TSa1& bsa1, TSa2& bsa2) {
//    
//}

template <typename TSymbol, typename TIndex, typename TBitSequenceArray>  
void CompactArrayBuilder<TSymbol, TIndex, TBitSequenceArray>::
copySymbolArray(CompactSymbolArray<TSymbol, TBitSequenceArray> &sa1, CompactSymbolArrayL<TSymbol> &sa2) {
//        // array size
//    size_t numOfSymbols;
//    // number of distinct symbols
//    size_t numOfDistinct;
//    // table of distinct symbols
//    TSymbol* symbolTable;
//
//    TBitSequenceArray indexes;    
//    // number of bits necessary to store an index
//    int bitsPerIndex;
    sa1.numOfSymbols = sa2.numOfSymbols;
    sa1.numOfDistinct = sa2.numOfDistinct;
    // copy symbol table
    sa1.freeTable();
    sa1.symbolTable = new TSymbol[sa1.numOfDistinct];
    for (int i = 0; i < sa1.numOfDistinct; ++i) 
        sa1.symbolTable[i] = sa2.symbolTable[i];
    // copy indexes    
    sa1.bitsPerIndex = sa2.bitsPerIndex;
    copyBitSeqArray(sa1.indexes, sa2.indexes, false);
}

#endif	/* COMPACTARRAYBUILDER_H */

