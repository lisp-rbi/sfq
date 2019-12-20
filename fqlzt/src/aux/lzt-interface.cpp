#include "lzt-interface.h"
#include "lzt-utils.h"

/**
 * Creates a trie from a list of words, lz-compresses it and saves it to file.
 * @param sortWords if true, words will be sorted lexicographically before compression
 * @return true if save operation is successful
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
    ofstream output(fname.c_str());
    CompactArraySerializer<TSymbol, TIndex> serializer(carray);
    serializer.arrayToStream(output);
    output.close();    
    delete carray;
    if (output.fail()) return false;
    else return true;
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
 * Single word retrieval is a special case. Empty prefix lists all words.
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
 * Deallocates all memory used by the trie structure.
 */
void freeTrieMemory(TLzTrie* trie) {
    delete trie;
}
