/**
 * CLI utility for testing the lzt-interface.
 * Code in interface function is also an example of using the lzt-interface.
 */

#include "lzt.hpp"
#include "lzt-interface.h"
#include "lzt-utils.h"

#include "lzt_core/util/regex.h"

#include <map>

using namespace std;

map<string, string> params;

/**************** INTERFACE FUNCTIONS ****************/
static void compressTrie();
static void loadAndListTrie();
static void queryTrie();
static void testInterfaceClass();
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
        else {
            cout << "unknown command";
        }
}

void compressTrie() {    
    vector<vector<TSymbol> >* words = readWordsFromFile(params["-i"]);
    createLzTrie(words, params["-d"], true);    
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
    vector<vector<TSymbol> >* words = readWordsFromFile(params["-i"]);    
    assert(lzt.make(words, params["-d"]));
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


