#ifndef COMPACTARRAYBUILDER_H
#define	COMPACTARRAYBUILDER_H

#include <cstddef>
#include <cassert>
#include <string>
#include <iostream>
#include <fstream>

#include "CompactArray.h"
#include "CompactSymbolArray.h"

#include "../../util/factory.h"
#include "../../util/filesystem_utils.h"
#include "../../dictionary/util/WordList.h"
#include "../vector_array/VectorArray.h"
#include "../compact_array_legacy/CompactArray.h"
#include "../compact_array_legacy/CompactSymbolArray.h"
#include "../compact_array_legacy/CompactArrayCreator.h"
#include "../compact_array_legacy/utils.h"
#include "../../serialization_legacy/BitSequence.h"
#include "../../serialization_legacy/serialization.h"
#include "../../serialization_legacy/SerializationUtils.h"
#include "../../serialization/BitSequenceArray.h"
#include "../../serialization/DiskCharArray.h"
#include "../../serialization/MemCharArray.h"
#include "../../util/utils.h"

using namespace std;

template <typename TSymbol, typename TIndex, typename TBitSequenceArray>
class CompactArrayBuilder {

public:

    typedef VectorArray<TSymbol, TIndex> TMemNodeArray;

    bool buildSaveCompactArray(WordList<TSymbol>* words, string folder,
                                string dictLabel, bool enumerated=false);

    CompactArray<TSymbol, TIndex, TBitSequenceArray> *
    createCompactArray(WordList<TSymbol>* words, string dictLabel, bool enumerated=false);

    CompactArray<TSymbol, TIndex, BitSequenceArray<MemCharArray> > *
    copyDiskArrayToMemArray(CompactArray<TSymbol, TIndex, BitSequenceArray<DiskCharArray> >* diskArray);

private:

    void copyFields(CompactArray<TSymbol, TIndex, TBitSequenceArray>& ca, CompactArrayL<TSymbol, TIndex>& cal);

    template<typename TCa1, typename TCa2>
    void copyFieldsGeneric(TCa1& ca, TCa2& cal);

    // slower version, using high-level interface for copying
    template<typename TBsa1, typename TBsa2>
    void copyBitSeqArrayHighlev(TBsa1& target, TBsa2& source, bool fieldsOnly);
    // faster version, copies char-array directly
    template<typename TCharArray>
    void copyBitSeqArrayBuffer(BitSequenceArray<TCharArray>& target, BitSequenceArrayL& source);

    template<typename TBSA1, typename TBSA2>
    void copyBitSeqArrayBufferGeneric(TBSA1& target, TBSA2& source, bool copyChars);

    template<typename TBsa1, typename TBsa2>
    void copyBitSeqArrayInPlace(TBsa1& target, TBsa2& source, string folder);

    //template<typename TSa1, typename TSa2>
    //void copySymbolArray(TSa1& bsa1, TSa2& bsa2);
    void copySymbolArray(CompactSymbolArray<TSymbol, TBitSequenceArray> &sa1,
                         CompactSymbolArrayL<TSymbol> &sa2, bool fieldsOnly);

    template<typename TSa1, typename TSa2>
    void copySymbolArrayGeneric(TSa1& target, TSa2& source, bool copyChars);

};

template <typename TSymbol, typename TIndex, typename TBitSequenceArray>
bool CompactArrayBuilder<TSymbol, TIndex, TBitSequenceArray>::
buildSaveCompactArray(WordList<TSymbol>* words, string folder, string dictLabel, bool enumerated) {
    //cout<<"buildSaveCompactArray(dict="<<dictLabel<<",enum="<<enumerated<<")"<<endl;
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
    bool res = carray->persist(folder);
    delete carray;
    return res;
}

template <typename TSymbol, typename TIndex, typename TBitSequenceArray>
CompactArray<TSymbol, TIndex, TBitSequenceArray>* CompactArrayBuilder<TSymbol, TIndex, TBitSequenceArray>::
createCompactArray(WordList<TSymbol>* words, string dictLabel, bool enumerated) {
    enumerated = (enumerated == true) ? (true) :(false);
    cout<<"createCompactArray(dict="<<dictLabel<<",enum="<<enumerated<<")"<<endl;
    TMemNodeArray* array = getLzArrayLCT<TMemNodeArray>(*words);
    CompactArrayCreatorL<TMemNodeArray> creator(*array);
    CompactArrayL<TSymbol, TIndex>* carrayLegacy = creator.createCompactArray();
    delete array;
    CompactArray<TSymbol, TIndex, TBitSequenceArray> *carray =
            new CompactArray<TSymbol, TIndex, TBitSequenceArray>();
    copyFields(*carray, *carrayLegacy);
    copyBitSeqArrayBuffer(carray->array, carrayLegacy->array);
    //copyBitSeqArrayHighlev(carray->array, carrayLegacy->array, false);
    copyBitSeqArrayBuffer(carray->siblings, carrayLegacy->siblings);
    //copyBitSeqArrayHighlev(carray->siblings, carrayLegacy->siblings, false);
    copyBitSeqArrayBuffer(carray->numOfWords, carrayLegacy->numOfWords);
    //copyBitSeqArrayHighlev(carray->numOfWords, carrayLegacy->numOfWords, false);
    copySymbolArray(carray->symbols, carrayLegacy->symbols, false);
    delete carrayLegacy;
    return carray;
}

template <typename TSymbol, typename TIndex, typename TBitSequenceArray>
CompactArray<TSymbol, TIndex, BitSequenceArray<MemCharArray> > *  CompactArrayBuilder<TSymbol, TIndex, TBitSequenceArray>::
copyDiskArrayToMemArray(CompactArray<TSymbol, TIndex, BitSequenceArray<DiskCharArray> >* diskArray) {
    CompactArray<TSymbol, TIndex, BitSequenceArray<MemCharArray> > *memArray =
            new CompactArray<TSymbol, TIndex, BitSequenceArray<MemCharArray> >();
    copyFieldsGeneric(*memArray, *diskArray);
    copyBitSeqArrayBufferGeneric(memArray->array, diskArray->array, false);
    copyBitSeqArrayBufferGeneric(memArray->siblings, diskArray->siblings, false);
    copyBitSeqArrayBufferGeneric(memArray->numOfWords, diskArray->numOfWords, false);
    copySymbolArrayGeneric(memArray->symbols, diskArray->symbols, false);
    return memArray;
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
template<typename TCa1, typename TCa2> void CompactArrayBuilder<TSymbol, TIndex, TBitSequenceArray>::
copyFieldsGeneric(TCa1& targetCa, TCa2& sourceCa) {
    targetCa.numOfDistinct = sourceCa.numOfDistinct;
    targetCa.numOfNodes = sourceCa.numOfNodes;
    targetCa.bitsPerIndex = sourceCa.bitsPerIndex;
    targetCa.enumerated = sourceCa.enumerated;
    assert(targetCa.NUM_OFFSETS == sourceCa.NUM_OFFSETS);
    for (int i = 0; i < targetCa.NUM_OFFSETS; ++i)
        targetCa.flagOffsets[i] = sourceCa.flagOffsets[i];
}

template <typename TSymbol, typename TIndex, typename TBitSequenceArray>
template<typename TBSA1, typename TBSA2> void CompactArrayBuilder<TSymbol, TIndex, TBitSequenceArray>::
copyBitSeqArrayBufferGeneric(TBSA1& target, TBSA2& source, bool copyChars) {
    target.numOfBlocks = source.numOfBlocks;
    target.numOfSequences = source.numOfSequences;
    target.bitsPerSequence = source.bitsPerSequence;
    target.charArray->setChars(source.charArray->getChars(), source.charArray->getNumChars(), copyChars);
}

template <typename TSymbol, typename TIndex, typename TBitSequenceArray>
template<typename TSa1, typename TSa2> void CompactArrayBuilder<TSymbol, TIndex, TBitSequenceArray>::
copySymbolArrayGeneric(TSa1& target, TSa2& source, bool copyChars) {
    target.numOfSymbols = source.numOfSymbols;
    target.numOfDistinct = source.numOfDistinct;
    // copy symbol table
    target.freeTable();
    target.symbolTable = new TSymbol[target.numOfDistinct];
    for (int i = 0; i < target.numOfDistinct; ++i)
        target.symbolTable[i] = source.symbolTable[i];
    // copy indexes
    target.bitsPerIndex = source.bitsPerIndex;
    copyBitSeqArrayBufferGeneric(target.indexes, source.indexes, copyChars);
}

template <typename TSymbol, typename TIndex, typename TBitSequenceArray>
template<typename TBsa1, typename TBsa2> void CompactArrayBuilder<TSymbol, TIndex, TBitSequenceArray>::
copyBitSeqArrayHighlev(TBsa1& target, TBsa2& source, bool fieldsOnly) {
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
template<typename TCharArray> void CompactArrayBuilder<TSymbol, TIndex, TBitSequenceArray>::
copyBitSeqArrayBuffer(BitSequenceArray<TCharArray>& target, BitSequenceArrayL& source) {
    target.numOfBlocks = source.numOfBlocks;
    target.numOfSequences = source.numOfSequences;
    target.bitsPerSequence = source.bitsPerSequence;
    //target.setCharArrayChars(source.blocks, source.numOfBlocks);
    target.charArray->setChars(source.blocks, source.numOfBlocks, true);
}

template <typename TSymbol, typename TIndex, typename TBitSequenceArray>
template<typename TBsa1, typename TBsa2> void CompactArrayBuilder<TSymbol, TIndex, TBitSequenceArray>::
copyBitSeqArrayInPlace(TBsa1& target, TBsa2& source, string folder) {
    if (accessible_filename(folder, "") == "") create_directory(folder);
    typedef typename TBsa1::ArrayType TArray;
    string fname = folder + "/" + TArray::PERSIST_CHARS_FNAME;
    TArray* diskArray = new TArray(fname);
    target.setCharArray(diskArray);
    copyBitSeqArrayBuffer(target, source);
    //copyBitSeqArrayHighlev(target, source, false);
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
    if (!fieldsOnly) {
        copyBitSeqArrayBuffer(sa1.indexes, sa2.indexes);
        // copyBitSeqArrayHighlev(sa1.indexes, sa2.indexes, false);
    }
}

#endif	/* COMPACTARRAYBUILDER_H */
