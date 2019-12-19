
#ifndef LZT_INTERFACE_H
#define LZT_INTERFACE_H

#include <cstdlib>
#include <cstddef>
#include <iostream>
#include <string>
#include <vector>

using namespace std;

#include "lzt_core/util/WordFileReader.h"
#include "lzt_core/util/factory.h"
#include "lzt_core/util/regex.h"
#include "lzt_core/util/Timer.h"
#include "lzt_core/dictionary/util/WordList.h"
#include "lzt_core/node_array/vector_array/VectorArray.h"
#include "lzt_core/node_array/compact_array/CompactArray.h"
#include "lzt_core/node_array/compact_array/CompactArrayCreator.h"
#include "lzt_core/serialization/array/CompactArraySerializer.h"
#include "lzt_core/node_array/util/CmmExporter.h"
#include "lzt_core/compress/stats/StatCreator.hpp"

/**************** DEFINITION OF TEMPLATE TYPES ****************/
typedef long TIndex;
typedef unsigned char TSymbol;
//typedef short int TSymbol;
//typedef int TSymbol;
typedef VectorArray<TSymbol, TIndex> TNodeArray;
typedef CompactArray<TSymbol, TIndex> TCompactArray;
typedef LzTrie<CompactArray<TSymbol, TIndex> > TLzTrie;
/**************************************************************/

/**************** INTERFACE FUNCTIONS ****************/
bool createLzTrie(vector<vector<TSymbol> >* words, string fname, bool sortWords = false);
TLzTrie* loadLzTrie(string trieFile);
/**************************************************************/

/**************** HELPER FUNCTIONS ****************/
WordList<TSymbol>* vecOfVec2WordList(vector<vector<TSymbol> >* words);
vector<vector<TSymbol> >* wordList2VecOfVec(WordList<TSymbol>* words);
vector<vector<TSymbol> >* readWordsFromFile(string file);
/**************************************************************/


#endif /* LZT_INTERFACE_H */

