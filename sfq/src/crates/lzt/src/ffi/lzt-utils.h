#ifndef LZT_UTILS_H
#define LZT_UTILS_H

#include "lzt-interface.h"
#include <string>

using namespace std;

/**************** WORD CONVERSIONS ****************/
// TODO create single interface (template) function and use
//   specialized version to handle various types
TSymbol* stringToTSymbolString(string str);
vector<TSymbol> string2SymbolVec(string s);
TSymbol* symbolVec2array(vector<TSymbol> w);
string symbolVec2string(vector<TSymbol> w);
/**************************************************************/

/**************** WORD SET CONVERSIONS ****************/
WordList<TSymbol>* vecOfVec2WordList(vector<vector<TSymbol> >* words);
vector<vector<TSymbol> >* wordList2VecOfVec(WordList<TSymbol>* words);


template <typename TSymbol> struct FlatWordList {   
    FlatWordList(TSymbol* w, long l): words(w), length(l) {}
    TSymbol *words;
    long length;
};
WordList<TSymbol>* flatwords2WordList(FlatWordList<TSymbol> fwords);
FlatWordList<TSymbol> wordList2Flatwords(WordList<TSymbol>* words);
/**************************************************************/

/**************** MISC HELPER FUNCTIONS ****************/
FlatWordList<TSymbol> readWordsFromFile(string file);
vector<vector<TSymbol> >* readWordsFromFileVecVec(string file);
/**************************************************************/

#endif /* LZT_UTILS_H */

