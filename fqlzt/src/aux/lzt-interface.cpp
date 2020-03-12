#include "lzt-interface.h"
#include "lzt-utils.h"
#include "lzt_core/util/filesystem_utils.h"

/**
 * Creates a trie from a list of words, lz-compresses it and saves it to folder.
 * @param sortWords if true, words will be sorted lexicographically before compression
 * @return true if save operation is successful
 */
bool createTrie(TSymbol* words, long length, string fname, bool sortWords) { 
    // derived from doCompress(string inputFile, string outputFile)
    // create lz-compressed trie, ie. array of nodes
    FlatWordList<TSymbol> fwords(words, length);
    WordList<TSymbol>* wlist = flatwords2WordList(fwords);
    if (sortWords) wlist->sort();
    CompactArrayBuilder<TSymbol, TIndex, TBitSeqArrayDisk> builder; // TODO add verbosity switch
    // create folder if it does not exist
    if (accessible_filename(fname, "") == "") {
        bool res = create_directory(fname);
        if (!res) return false;
    }
    bool res = builder.buildSaveCompactArray(wlist, fname, ""); 
    delete wlist;        
    return res;
}

/**
 * Load lz-compressed and compactified trie from a file.
 * @return pointer to lz-trie or NULL if loading failed
 */
TLzTrie* loadTrie(string trieFolder, bool mem) {
    TCompactArrayDisk* nodeArrayDisk = new TCompactArrayDisk();
    TCompactArrayMem* nodeArrayMem = NULL;
    TCompactArray* nodeArray = NULL;
    //cout<<"Trie folder:"<<trieFolder<<endl;
    nodeArrayDisk->load(trieFolder); 
    if (mem) {
        CompactArrayBuilder<TSymbol, TIndex, TCompactArrayDisk> builder;
        nodeArrayMem = builder.copyDiskArrayToMemArray(nodeArrayDisk);
        delete nodeArrayDisk;
        nodeArrayMem->setCache(10000);
        nodeArray = nodeArrayMem;
    }
    else {
        nodeArrayDisk->setCache(10000);
        nodeArray = nodeArrayDisk;
    }    
    TLzTrie* trie = new TLzTrie(*nodeArray);
    return trie;
}

/**
 * Return a list of words in the trie with query as prefix. 
 * Single word retrieval is a special case. Empty prefix lists all words.
 */
vector<vector<TSymbol> >* queryTrie(TLzTrie* trie, vector<TSymbol> query) {
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
void freeTrieMem(TLzTrie* trie) {
    delete trie;
}
