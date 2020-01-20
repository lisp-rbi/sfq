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
/**************************************************************/

/**************** MISC HELPER FUNCTIONS ****************/
vector<vector<TSymbol> >* readWordsFromFile(string file);
/**************************************************************/

#endif /* LZT_UTILS_H */

