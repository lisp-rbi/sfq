
#include "lzt-interface.h"
#include "lzt-utils.h"
#include "lzt_core/util/regex.h"


using namespace std;

#include <iostream>
#include <cstddef>
#include <vector>
#include <iterator>
#include <algorithm>
#include <numeric>
class Lzt {

  public:

    vector < TSymbol> objvec;

    Lzt(string Path);
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


  private:
    vector< vector < TSymbol> >* getFastqRecords(vector<TSymbol> prefix);

    TLzTrie *trie = NULL;


};
