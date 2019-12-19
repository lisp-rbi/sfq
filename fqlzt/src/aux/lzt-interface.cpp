#include "lzt-interface.h"

/**
 * Creates a trie from a list of words, lz-compresses it and saves it to file.
 * @param sortWords if true, words will be sorted lexicographically before compression
 * @return true if operation is successful
 */
bool createLzTrie(vector<vector<TSymbol> >* words, string fname, bool sortWords) { 
    // derived from doCompress(string inputFile, string outputFile)
    // create lz-compressed trie, ie. array of nodes
    WordList<TSymbol>* wlist = vecOfVec2WordList(words);
    if (sortWords) wlist->sort();
    TNodeArray* array = getLzArrayLCT<TNodeArray>(*wlist);
    delete wlist;        
    // build compact array
    CompactArrayCreator<TNodeArray> compacter(*array);
    TCompactArray* carray =  compacter.createCompactArray();
    delete array;
    // serialize compact array to file
    // TODO I/O checking: is file writeable
    ofstream output(fname.c_str());
    CompactArraySerializer<TSymbol, TIndex> serializer(carray);
    serializer.arrayToStream(output);
    output.close();
    delete carray;
    return 1;
}

/**
 * Load lz-compressed and compactified trie from a file.
 * @return pointer to lz-trie or NULL if loading failed
 */
TLzTrie* loadLzTrie(string trieFile) {
        TLzTrie* lzTrie = getLzTrieFromCompressedFile<TSymbol, TIndex>(trieFile);
        return lzTrie;
}

/**
 * Return a list of words in the trie with query as prefix. 
 * Single word retrieval is a special case.
 */
vector<vector<TSymbol> >* queryLzTrie(TLzTrie* trie, vector<TSymbol> query) {
    TSymbol* nativeQuery = symbolVec2array(query);
    WordList<TSymbol>* words = trie->getWordsByPrefix(nativeQuery);
    vector<vector<TSymbol> >* result = wordList2VecOfVec(words);
    delete words;
    delete [] nativeQuery;
    return result;
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

