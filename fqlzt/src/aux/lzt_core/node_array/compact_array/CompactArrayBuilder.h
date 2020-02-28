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
#include "util/filesystem_utils.h"
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

    typedef VectorArray<TSymbol, TIndex> TMemNodeArray;
    
    bool buildSaveCompactArray(WordList<TSymbol>* words, string folder, 
                                string dictLabel, bool enumerated=false);
    
    CompactArray<TSymbol, TIndex, TBitSequenceArray> *
    createCompactArray(WordList<TSymbol>* words, string dictLabel, bool enumerated=false);

private:    
    
    void copyFields(CompactArray<TSymbol, TIndex, TBitSequenceArray>& ca, CompactArrayL<TSymbol, TIndex>& cal);
    
    template<typename TBsa1, typename TBsa2>
    void copyBitSeqArray(TBsa1& bsa1, TBsa2& bsa2, bool fieldsOnly);
    
    template<typename TBsa1, typename TBsa2>
    void copyBitSeqArrayInPlace(TBsa1& bsa1, TBsa2& bsa2, string folder);    
    
    //template<typename TSa1, typename TSa2>
    //void copySymbolArray(TSa1& bsa1, TSa2& bsa2);
    void copySymbolArray(CompactSymbolArray<TSymbol, TBitSequenceArray> &sa1, 
                         CompactSymbolArrayL<TSymbol> &sa2, bool fieldsOnly);

};

template <typename TSymbol, typename TIndex, typename TBitSequenceArray>
bool CompactArrayBuilder<TSymbol, TIndex, TBitSequenceArray>::
buildSaveCompactArray(WordList<TSymbol>* words, string folder, string dictLabel, bool enumerated) {    
    cout<<"buildSaveCompactArray(dict="<<dictLabel<<",enum="<<enumerated<<")"<<endl;
    TMemNodeArray* array = getLzArrayLCT<TMemNodeArray>(*words);
    CompactArrayCreatorL<TMemNodeArray> creator(*array);
    CompactArrayL<TSymbol, TIndex>* carrayLegacy = creator.createCompactArray();    
    delete array;    
    CompactArray<TSymbol, TIndex, TBitSequenceArray> *carray = 
        new CompactArray<TSymbol, TIndex, TBitSequenceArray>();  
    copyFields(*carray, *carrayLegacy);    
    string arrayFolder =  folder+"/"+CompactArray<TSymbol, TIndex, TBitSequenceArray>::ARRAY_FOLDER;        
    copyBitSeqArrayInPlace(carray->array, carrayLegacy->array, arrayFolder);    
    string siblingsFolder =  folder+"/"+CompactArray<TSymbol, TIndex, TBitSequenceArray>::SIBLINGS_FOLDER;
    copyBitSeqArrayInPlace(carray->siblings, carrayLegacy->siblings, siblingsFolder);    
    string numwFolder =  folder+"/"+CompactArray<TSymbol, TIndex, TBitSequenceArray>::NUMOFWORDS_FOLDER;
    copyBitSeqArrayInPlace(carray->numOfWords, carrayLegacy->numOfWords, numwFolder);    
    string symbarrFolder =  folder+"/"+CompactArray<TSymbol, TIndex, TBitSequenceArray>::SYMBOLS_FOLDER;
    copySymbolArray(carray->symbols, carrayLegacy->symbols, true);        
    copyBitSeqArrayInPlace(carray->symbols.indexes, carrayLegacy->symbols.indexes, symbarrFolder);    
    delete carrayLegacy;   
    // !!! PODFOLDERI SA STRUKTURAMA SU PRAZNI!
    // !!! OSTAJU FAJLOVI OD TMP CHAR ARRAY-a - POČISTITI
    bool res = carray->persist(folder);      
    delete carray;
    return res;
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
    copySymbolArray(carray->symbols, carrayLegacy->symbols, false);
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
copyBitSeqArray(TBsa1& target, TBsa2& source, bool fieldsOnly) {
    if (fieldsOnly) {
        target.numOfBlocks = source.numOfBlocks;
        target.numOfSequences = source.numOfSequences;
        target.bitsPerSequence = source.bitsPerSequence;
    }
    else {
        target.changeFormat(source.getNumOfSequences(), source.getSequenceSize());
        for (size_t i = 0; i < source.getNumOfSequences(); ++i) 
            target.setSequence(i, source[i]);        
    }
}

template <typename TSymbol, typename TIndex, typename TBitSequenceArray>
template<typename TBsa1, typename TBsa2> void CompactArrayBuilder<TSymbol, TIndex, TBitSequenceArray>::    
copyBitSeqArrayInPlace(TBsa1& target, TBsa2& source, string folder) {
    if (accessible_filename(folder, "") == "") create_directory(folder);
    typedef typename TBsa1::ArrayType TArray;
    string fname = folder + "/" + TArray::PERSIST_CHARS_FNAME;        
    TArray* diskArray = new TArray(fname);        
    target.setCharArray(diskArray);    
    copyBitSeqArray(target, source, false);    
}

template <typename TSymbol, typename TIndex, typename TBitSequenceArray>  
void CompactArrayBuilder<TSymbol, TIndex, TBitSequenceArray>::
copySymbolArray(CompactSymbolArray<TSymbol, TBitSequenceArray> &sa1, CompactSymbolArrayL<TSymbol> &sa2, 
                    bool fieldsOnly) {
    sa1.numOfSymbols = sa2.numOfSymbols;
    sa1.numOfDistinct = sa2.numOfDistinct;
    // copy symbol table
    sa1.freeTable();
    sa1.symbolTable = new TSymbol[sa1.numOfDistinct];
    for (int i = 0; i < sa1.numOfDistinct; ++i) 
        sa1.symbolTable[i] = sa2.symbolTable[i];
    // copy indexes    
    sa1.bitsPerIndex = sa2.bitsPerIndex;
    if (!fieldsOnly) copyBitSeqArray(sa1.indexes, sa2.indexes, false);
}

#endif	/* COMPACTARRAYBUILDER_H */

