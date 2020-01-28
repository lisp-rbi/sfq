/**
 * CLI utility for testing the lzt-interface.
 * Code in interface function is also an example of using the lzt-interface.
 */

#include "lzt.hpp"
#include "lzt-interface.h"
#include "lzt-utils.h"

#include "lzt_core/util/regex.h"

#include <cstdlib>
#include <ctime>
#include <map>

using namespace std;

map<string, string> params;

/**************** INTERFACE FUNCTIONS ****************/
static void compressTrie();
static void loadAndListTrie();
static void queryTrie();
static void testInterfaceClass();
static void testSequentialQueries();
/**************************************************************/

static void createParameterMap(int argc, char** argv);
static void printWordList(string query, TLzTrie* lzTrie);

int main(int argc, char** argv) {        
    string command = argv[1];        
    createParameterMap(argc-2, argv+2);
        if (command == "c") {
            compressTrie();
        }
        else if (command == "l") {
            loadAndListTrie();
        }
        else if (command == "s") {
            queryTrie();
        }
        else if (command == "cls") {
            testInterfaceClass();
        }
        else if (command == "sq") {
            testSequentialQueries();
        }
        else {
            cout << "unknown command";
        }
}

void compressTrie() {    
    FlatWordList<TSymbol> fwords = readWordsFromFile(params["-i"]);        
    createLzTrie(fwords.words, fwords.length, params["-d"]);    
    delete [] fwords.words;
}

void loadAndListTrie() {        
    TLzTrie* trie = loadLzTrie(params["-d"]);
    string query = "*";
    printWordList(query, trie);
    freeTrieMemory(trie);
}

void queryTrie() {        
    TLzTrie* trie = loadLzTrie(params["-d"]);
    string query = params["-s"];
    vector<TSymbol> q = string2SymbolVec(query);
    vector<vector<TSymbol> >* result = queryLzTrie(trie, q);
    for(size_t i = 0; i < result->size(); ++i) {        
        cout<<symbolVec2string((*result)[i])<<endl;
    }
    freeTrieMemory(trie);
}

/** 
 * Test Lzt class interface by creating a trie, 
 * loading it, and listing all words to stdout. 
 */
void testInterfaceClass() {
    Lzt lzt;
    // create and save    
    FlatWordList<TSymbol> fwords = readWordsFromFile(params["-i"]);    
    assert(lzt.make(fwords.words, fwords.length, params["-d"]));
    delete [] fwords.words;
    // load
    assert(lzt.read(params["-d"]));
    // list all words
    vector<TSymbol> prefix; // empty prefix
    vector<vector<TSymbol> >* result = lzt.getFastqRecords(prefix);   
    assert(result != NULL);
    for(size_t i = 0; i < result->size(); ++i) {        
        cout<<symbolVec2string((*result)[i])<<endl;
    }
}

/**
 * Test scenario when a trie is loaded and queried multiple (many) times.
 * Run several runs (-n cli param) of queries of all words in the trie, checking the result, 
 * and interleave word queries with list-all-words and list-by-prefix queries.
 */
void testSequentialQueries() {    
    // load trie and list all words
    TLzTrie* trie = loadLzTrie(params["-d"]);    
    vector<TSymbol> emptyQuery;
    vector<vector<TSymbol> >* allwords = queryLzTrie(trie, emptyQuery);
    size_t numWords = allwords->size();
    // perform testing runs
    int numRuns = 1;
    // how often to perform list-all and list-prefix queries
    // decision is random, expected to happen once in this many words
    int listallFreq = 5000, listPrefix = 500;
    if (params.count("-n") > 0) numRuns = atoi(params["-n"].c_str());
    srand(time(0));
    for (int i = 0; i < numRuns; ++i) {
        cout << "RUN " << i+1 << endl;
        for (size_t j = 0; j < numWords; ++j) {            
            vector<TSymbol> wrd = allwords->at(j); // get j-th words
            // perform random list all and list by prefix tests
            int r =rand();
            if (r % listallFreq == 0) {
                //cout<<"listall"<<endl;
                vector<vector<TSymbol> >* words = queryLzTrie(trie, emptyQuery);
                delete words;
            }            
            if (wrd.size() > 1 and r % listPrefix == 0) {
                // prefix size should be between 1 and wrd.size-1
                int prefixLen = (rand()%(wrd.size()-1))+1;               
//                vector<TSymbol>::iterator first = 
//                vector<TSymbol>::iterator last = 
                vector<TSymbol> q(wrd.begin(), wrd.begin()+prefixLen);
                //cout<<"listbyprefix:["<<symbolVec2string(q)<<"]"<<endl;
                vector<vector<TSymbol> >* words = queryLzTrie(trie, q);
                delete words;
            }
            vector<vector<TSymbol> >* result = queryLzTrie(trie, wrd);
            // can return more than one word if its a prefix of another            
            assert(result->size() > 0); 
            bool found = false;
            for (size_t k = 0; k < result->size(); ++k) {
                if (wrd == result->at(k)) {
                    found = true;
                    break;
                }
            }
            assert(found);
            delete result;
        }
    }
    delete allwords;
    freeTrieMemory(trie);
}

void printWordList(string query, TLzTrie* lzTrie) {
    // convery string of chars to string of TSymbols
    TSymbol *queryTS = stringToTSymbolString(query);
    //cout<<"start printing"<<endl;
    if (query.find('*') != string::npos) { // list words
        TSymbol prefix[1000];      
        bool syntaxOk = getPrefixBeforeStar<TSymbol>(queryTS, prefix);
        //cout<<"prefix calculated"<<endl;
        if (syntaxOk) {            
            WordList<TSymbol>* list = lzTrie->getWordsByPrefix(prefix);
            wordListToStreamChars<TSymbol>(list, cout);
            //cout<<"number of words: "<<list->numberOfWords()<<endl;
            delete list;
        }
        else {
            cout<<"* symbol must be the last symbol of the input."<<endl;
        }
    }
    else { // print a single word
        bool contains = lzTrie->containsWord(queryTS);
        if (contains) cout<<"in the dictionary"<<endl;
        else cout<<"NOT in the dictionary"<<endl;
    }

    delete [] queryTS;
}

/**
 * Scann command line params and structure them into local variable params (stl map)
 */
void createParameterMap(int argc, char** argv) {
    for (int i = 0; i < argc; ++i) {
        string arg = argv[i];
        // arguments for wich parameters are expected
        if (arg == "-i" || arg == "-d" || arg == "-s" || 
                arg == "-a" || arg == "-f" || arg == "-o" || arg == "-r" || arg == "-n") {
            if (i < argc - 1) {
                string val = argv[i+1];
                if (val.size() > 0 && val[0] != '-') params[arg] = val;
            }
        } else {
            // flag arguments
            if (arg == "-cmm") params[arg] = "rjecnik.cmm";
            else if (arg == "-st") params[arg] = "true";
            else if (arg == "-t") params[arg] = "true";
            else if (arg == "-e") params[arg] = "true";
            else if (arg == "-l") params[arg] = "true";
            else if (arg == "-c") params[arg] = "true";
            else if (arg == "-z") params[arg] = "true";
        }
    }
}


