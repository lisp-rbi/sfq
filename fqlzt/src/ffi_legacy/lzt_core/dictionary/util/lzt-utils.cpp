#include "lzt-utils.h"

/**
 * Converts cpp string to array of TSymbols.
 */
TSymbol* stringToTSymbolString(string str) {
    TSymbol *tss = new TSymbol[str.size()+1];
    int i;
    for (i = 0; i < str.size(); ++i) tss[i] = (TSymbol)str[i];
    tss[i] = zeroSymbol<TSymbol>();
    return tss;
}

/**
 * Converts cpp string to vector of TSymbols.
 */
vector<TSymbol> string2SymbolVec(string s) {
    vector<TSymbol> sv(s.size());
    for (int i = 0; i < s.size(); ++i) sv[i] = (TSymbol)s[i];
    return sv;
}

/**
 * Converts word (sequence of TSymbols) from vector to array.
 */
TSymbol* symbolVec2array(vector<TSymbol> w) {
    TSymbol* nw = new TSymbol[w.size()+1];        
    size_t j;
    for (j = 0; j < w.size(); ++j) nw[j] = w[j];
    nw[j] = zeroSymbol<TSymbol>();
    return nw;
}

/**
 * Converts word (sequence of TSymbols) from vector to string.
 */
string symbolVec2string(vector<TSymbol> w) {
    string s;
    for (int j = 0; j < w.size(); ++j) s.push_back(w[j]);    
    return s;
}

/** 
 * Converts set of words represented as vector of symbol vectors 
 * to WordList object used as input for trie creation. 
 */
WordList<TSymbol>* vecOfVec2WordList(vector<vector<TSymbol> >* words) {
    WordList<TSymbol>* wlist = new WordList<TSymbol>();    
    for(size_t i = 0; i < words->size(); ++i) {                
        vector<TSymbol> w = (*words)[i];
        wlist->addWord(symbolVec2array(w));  
        //cout<<nw<<endl;
    }
    return wlist;
}

/**
 * Converts WordList to vector of vectors representation. 
 * This is for testing purposes, since loaders in the library produce WordLists 
 * and fasta interface methods use vectors of vectors
 */
vector<vector<TSymbol> >* wordList2VecOfVec(WordList<TSymbol>* words) {
    vector<vector<TSymbol> >* vvwords = new vector<vector<TSymbol> >;
    for (size_t i = 0; i < words->numberOfWords(); ++i) {
        TSymbol const * s = (*words)[i];        
        vector<TSymbol> vword;        
        for (size_t j = 0; s[j] != 0; ++j)
            vword.push_back(s[j]);        
        vvwords->push_back(vword);
    }
    return vvwords;
}

/**
 * Reads words from word-per-line txt file to vector-of-vectors format.
 */
vector<vector<TSymbol> >* readWordsFromFile(string file) {
    WordFileReader<TSymbol> reader(file);
    WordList<TSymbol>* words = reader.getWords();
    vector<vector<TSymbol> >* vvwords = wordList2VecOfVec(words);
    for(size_t i = 0; i < vvwords->size(); ++i) {                
        vector<TSymbol> w = (*vvwords)[i];
//        for(int j = 0; j < w.size(); ++j) cout << w[j];
//        cout<<endl;
    }
    delete words;
    return vvwords;
}

