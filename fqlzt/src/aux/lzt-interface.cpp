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
 * Converts set of words represented as vector of symbol vectors 
 * to WordList object used as input for trie creation. 
 */
WordList<TSymbol>* vecOfVec2WordList(vector<vector<TSymbol> >* words) {
    WordList<TSymbol>* wlist = new WordList<TSymbol>();    
    for(size_t i = 0; i < words->size(); ++i) {                
        vector<TSymbol> w = (*words)[i];
        TSymbol* nw = new TSymbol[w.size()+1];        
        size_t j;
        for (j = 0; j < w.size(); ++j) nw[j] = w[j];
        nw[j] = zeroSymbol<TSymbol>();
        wlist->addWord(nw);  
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

//void printWordList(string query, LzTrie<CompactArray<TSymbol, TIndex> >* lzTrie) {
//    // convery string of chars to string of TSymbols
//    TSymbol *queryTS = stringToTSymbolString(query);
//    //cout<<"start printing"<<endl;
//    if (query.find('*') != string::npos) { // list words
//        TSymbol prefix[1000];      
//        bool syntaxOk = getPrefixBeforeStar<TSymbol>(queryTS, prefix);
//        //cout<<"prefix calculated"<<endl;
//        if (syntaxOk) {            
//            WordList<TSymbol>* list = lzTrie->getWordsByPrefix(prefix);
//            wordListToStreamChars<TSymbol>(list, cout);
//            //cout<<"number of words: "<<list->numberOfWords()<<endl;
//            delete list;
//        }
//        else {
//            cout<<"* symbol must be the last symbol of the input."<<endl;
//        }
//    }
//    else { // print a single word
//        bool contains = lzTrie->containsWord(queryTS);
//        if (contains) cout<<"in the dictionary"<<endl;
//        else cout<<"NOT in the dictionary"<<endl;
//    }
//
//    delete [] queryTS;
//}
//
//void search() {
//    if (params.count("-d") == 0) {
//        cout<<"Please specify the dictionary file." << endl;
//        printUsage();
//        return;
//    }
//    bool stdInput = (params.count("-s") == 0);
//
//    LzTrie<CompactArray<TSymbol, TIndex> >* lzTrie = 
//            getLzTrieFromCompressedFile<TSymbol, TIndex>(params["-d"]);
//    //cout<<"trie loaded"<<endl;
//
//    if (stdInput) {
//        string input;
//        while (true) {
//            cin >> input;
//            if (input == "^") break;
//            printWordList(input, lzTrie);
//        }
//    }
//    else {
//        // replace # by * in the query string
////        string query = params["-s"];
////        for (int i = 0; i < query.size(); ++i)
////            if (query[i] == '#') query[i] = '*';
//
//        printWordList(params["-s"], lzTrie);
//    }
//
//    delete lzTrie;
//}
//
//
//TSymbol* stringToTSymbolString(string& str) {
//    TSymbol *tss = new TSymbol[str.size()+1];
//    int i;
//    for (i = 0; i < str.size(); ++i) tss[i] = (TSymbol)str[i];
//    tss[i] = zeroSymbol<TSymbol>();
//    return tss;
//}


