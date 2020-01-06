
#include "lzt-interface.h"
#include "lzt-utils.h"
#include "lzt_core/util/regex.h"


using namespace std;

#include <iostream>
#include <cstddef>
#include <vector>

class Lzt {

  public:

    Lzt();
    ~Lzt();

    /**
     * Creates compressed trie from a sorted list of words
     * assumes dict does not exist and overwrites it
     */
    static bool make(vector<vector<TSymbol> >* words, string savePath);

    /**
     *  If trie exists, load the data structure into self
     *  Return true if loading is successful, else false.
     */
    bool read(string triePath);

    /**
     * Query (loaded) trie by prefix, return a list of all words sharing the prefix.
     * Empty prefix lists all words.
     */
    vector< vector < TSymbol> >* getFastqRecords(vector<TSymbol> prefix);

  private:

    TLzTrie *trie = NULL;

};
