#include "lzt.hpp"
#include "lzt-interface.h"

#include <map>

using namespace std;

map<string, string> params;

static void createParameterMap(int argc, char** argv);
static void compressTrie();
static void loadAndListTrie();
static void printWordList(string query, TLzTrie* lzTrie);
static TSymbol* stringToTSymbolString(string& str);

int main(int argc, char** argv) {        
    string command = argv[1];        
    createParameterMap(argc-2, argv+2);
        if (command == "c") {
            compressTrie();
        }
        else if (command == "l") {
            loadAndListTrie();
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

TSymbol* stringToTSymbolString(string& str) {
    TSymbol *tss = new TSymbol[str.size()+1];
    int i;
    for (i = 0; i < str.size(); ++i) tss[i] = (TSymbol)str[i];
    tss[i] = zeroSymbol<TSymbol>();
    return tss;
}

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


