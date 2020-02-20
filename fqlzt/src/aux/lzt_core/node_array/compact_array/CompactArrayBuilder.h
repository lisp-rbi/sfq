#ifndef COMPACTARRAYBUILDER_H
#define	COMPACTARRAYBUILDER_H

#include <cstddef>
#include <cassert>
#include <string>
#include <iostream>
#include <fstream>

#include "CompactArray.h"

#include "util/factory.h"
#include "dictionary/util/WordList.h"
#include "node_array/vector_array/VectorArray.h"
#include "node_array/compact_array_legacy/CompactArray.h"
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

private:

    typedef VectorArray<TSymbol, TIndex> TMemNodeArray;
    
    //TNodeArray const & nodeArray;
    //CompactArrayL<TSymbol, TIndex>* compactArray;        

};

template <typename TSymbol, typename TIndex, typename TBitSequenceArray>
void CompactArrayBuilder<TSymbol, TIndex, TBitSequenceArray>::
buildSaveCompactArray(WordList<TSymbol>* words, string folder, string dictLabel, bool enumerated) {    
    cout<<"buildSaveCompactArray(dict="<<dictLabel<<",enum="<<enumerated<<")"<<endl;
    TMemNodeArray* array = getLzArrayLCT<TMemNodeArray>(*words);
    CompactArrayCreatorL<TMemNodeArray> creator(*array);
    CompactArrayL<TSymbol, TIndex>* carrayLegacy = creator.createCompactArray();
    delete array;
    delete carrayLegacy;
}

#endif	/* COMPACTARRAYBUILDER_H */

