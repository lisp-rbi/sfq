#ifndef LZTRIE_H
#define	LZTRIE_H

#include <stack>
#include <vector>
#include <cassert>
#include <cstddef>

#include "../../util/utils.h"
#include "../util/WordList.h"
#include "../ILexicon.h"
#include "../../node_array/na_utils.h"

template <typename TNodeArray>
class LzTrie : public ILexicon<typename TNodeArray::Symbol> {
private:
    typedef typename TNodeArray::Symbol TSymbol;
    typedef typename TNodeArray::Index TIndex;

public:

    typedef typename TNodeArray::NodeConst TNodeConst;

    LzTrie(const TNodeArray& nodes);
    virtual ~LzTrie();

    bool containsWord(TSymbol const * word);
    WordList<TSymbol>* getWordsByPrefix(TSymbol const * word, long maxWords=0, bool diag=false);

private:

    /** for recursive trie searching, data about a location where
     * search moved when a pointer was followed */
    struct ArraySegment {
        // current position in the segment
        TIndex pos;
        // end of segment, this position is included in the search
        TIndex end;

        ArraySegment(): pos(0), end(0) {}
        ArraySegment(TIndex p, TIndex e): pos(p), end(e) {}
        void set(TIndex p, TIndex e) { pos = p; end = e; }
    };

    /** Describes what changes were made to LzStack of ArraySegments when
     * trie traversal moved from a node to it's sibling or successor node. */
    struct LzStackChange {
        int numAdded;
        int numRemoved;
        // value of the top position before increase
        TIndex topPosition;
    };

    bool diag; // flag for diagnostic messages print
    long maxWords; // max words to extract
    bool stopSearch; // flag to stop search by prefix

    const TNodeArray& nodes;
    // size of the array
    TIndex N;
    // results of the searchWord
    // word is in the trie
    bool wordFound;
    // word is a path in the trie, but does not necessarily end in accepting state
    bool wordEnded;

    /** If true, all the changes to lzStack during traversal will be recorded
     * so that they can be reversed. */
    bool recordChanges;

    /* Stack with nested array segements created by following pointers,
     * it describes a position in the lz-compressed node array. */
    stack<ArraySegment> lzStack;
    // to reverse changes to lzStack, popped segments have to be stored
    stack<ArraySegment> lzStackPopped;
    stack<LzStackChange> lzStackChanges;

    /* buffer for word that is the path from root to current node visited by DFS */
    vector<TSymbol> wordBuffer;
    // list of words that are
    WordList<TSymbol>* wordList;
    // word whose suffixes are being searched
    TSymbol const * prefix;
    size_t prefixLength;

    void searchWord(TSymbol const * word);
    void depthFirstSearch(TIndex offset);
    void beforeVisit(TNodeConst node);
    void afterVisit(TNodeConst node);
    void reverseStackChange(LzStackChange c);

    template<typename TIterator>
    bool compressedContains(TIterator word) const;

    void moveToNode(TIndex inc);
    int followPointers();
    int removeUsedSegments();

    void printStack();
    void clearStack();

};

template <typename TNodeArray>
LzTrie<TNodeArray>::LzTrie(const TNodeArray& n)
: nodes(n), N(n.getSize()) { }

template <typename TNodeArray>
LzTrie<TNodeArray>::~LzTrie() { delete &nodes; }

template <typename TNodeArray>
bool LzTrie<TNodeArray>::containsWord(TSymbol const * word) {
//    searchWord(word);
//    return wordFound;
    return compressedContains(word);
}

/** Get list of all words in the trie with prefix word. */
template <typename TNodeArray> WordList<typename TNodeArray::Symbol>*

/// RB long maxWords=0, bool diag=false
LzTrie<TNodeArray>::getWordsByPrefix(TSymbol const * word, long maxWords, bool diag) {
    // find a node whose subtree contains the suffixes
    this->diag = diag; this->maxWords = maxWords;
    searchWord(word);
    if (diag) cout<<"searchWord completed for word: ["<<word<<"]"<<endl;
    bool prefixEmpty = (word[0] == zeroSymbol<TSymbol>());
    // check if there are words to search in the trie
    if (!prefixEmpty) {
        // if the word does not exist in the trie as a prefix, end
        if (wordEnded == false) {
            clearStack();
            return NULL;
        }

        /* Check if there are any suffixes. After the trie search with the word,
         * stack will describe a position of the last symbol symbol matched. */
        TNodeConst n = nodes[lzStack.top().pos];
        if (n.getCow() == false) {
            if (wordFound) {
                // no continuation, word found -> result is only the word
                WordList<TSymbol>* list = new WordList<TSymbol>();
                list->addWord(word);
                clearStack();
                return list;
            }
            else {
                /* This case should not happen, word whose symbols are a path
                 * in the trie, there's no continuation and no ending at last char. */
                assert(false);
                return NULL;
            }
        }
    }

    recordChanges = true;
    prefix = word;
    prefixLength = wordLength(prefix);
    wordList = new WordList<TSymbol>();
    if (wordFound) wordList->addWord(word);

    // offset to the node from which the depth first search will start
    TIndex offset;
    if (!prefixEmpty) offset = 1;
    else offset = 0;
    if (diag) cout<<"starting depthFirstSearch"<<endl;
    stopSearch = false;
    depthFirstSearch(offset);

    wordBuffer.clear();
    clearStack();
    return wordList;
}

/** Starting from the node at offset from the current (lzStack) position
 * in the trie, do a depth first search on the subtree, recording the visited words. */
template <typename TNodeArray>
void LzTrie<TNodeArray>::depthFirstSearch(TIndex offset) {
//    TIndex oldPos = lzStack.top().pos;
  //  printStack(); cout << offset << endl;
    if (stopSearch) return;
    // move lzStack to current node, recording changes to the lzStack
    if (diag) cout<<"DFS;";
    if (offset != 0) moveToNode(offset);

    TNodeConst n = nodes[lzStack.top().pos];
    //cout<<n.getSymbol()<<endl;
    assert(n.isPointer() == false);
    beforeVisit(n);
    //if (stopSearch) return;

    // visit subtree
    if (n.getCow()) depthFirstSearch(1);
    afterVisit(n);
    // visit siblings
    if (n.getSibling() != 0) depthFirstSearch(n.getSibling());

    // reverse changes to the lzStack
    if (offset != 0) {
        reverseStackChange(lzStackChanges.top());
        lzStackChanges.pop();
//        lzStack.top().pos = oldPos;
    }
}

template <typename TNodeArray>
inline void LzTrie<TNodeArray>::beforeVisit(TNodeConst node) {
    if (diag) cout<<"BV;";
    wordBuffer.push_back(node.getSymbol());
    if (node.getEow() == false) return;
    // add word ending with current node to word list
    size_t wordLength = prefixLength + wordBuffer.size();
    TSymbol word[wordLength + 1];
    for (size_t i = 0; i < prefixLength; ++i) word[i] = prefix[i];
    for (size_t i = 0; i < wordBuffer.size(); ++i)
        word[prefixLength+i] = wordBuffer[i];
    word[wordLength] = zeroSymbol<TSymbol>();
    //cout<<"w:["<<word<<"]"<<endl;
    wordList->addWord(word);
    if (diag) cout<<wordList->numberOfWords()<<endl;
    if (maxWords > 0 && wordList->numberOfWords() >= maxWords) stopSearch=true;
}

template <typename TNodeArray>
inline void LzTrie<TNodeArray>::afterVisit(TNodeConst node) {
    if (diag) cout<<"AV;";
    wordBuffer.pop_back();
}

/** Move to a node at offset + pos, where pos is position on top of the stack.
 * This implements move to a sibling and successor.
 * LzStack is changed to new position, by removing segments whose end is
 * reached, and by adding segments if pointers are followed. Changes are
 * recorded if specified. */
template <typename TNodeArray>
inline void LzTrie<TNodeArray>::moveToNode(TIndex offset) {
    if (diag) cout<<"MOV;";
    LzStackChange change;
    change.topPosition = lzStack.top().pos;

    lzStack.top().pos += offset;
    ArraySegment top = lzStack.top();

    if (top.pos <= top.end) { // we're still in the same segment
        change.numRemoved = 0;
        TNodeConst n = nodes[top.pos];
        if (n.isPointer()) change.numAdded = followPointers();
        else change.numAdded = 0;
    }
    else { // end of segment reached
        assert(top.pos == top.end + 1);
        change.numRemoved = removeUsedSegments();
        top = lzStack.top();
        TNodeConst n = nodes[top.pos];
        if (n.isPointer()) change.numAdded = followPointers();
        else change.numAdded = 0;
    }

    if (recordChanges) lzStackChanges.push(change);
}

/** Reverse change to lzStack.  */
template <typename TNodeArray>
inline void LzTrie<TNodeArray>::reverseStackChange(LzStackChange c) {
    if (diag) cout<<"REV;";
    if (c.numAdded) {
        for (int i = 0; i < c.numAdded; ++i) lzStack.pop();
    }
    if (c.numRemoved) {
        for (int i = 0; i < c.numRemoved; ++i) {
            ArraySegment s = lzStackPopped.top();
            lzStackPopped.pop();
            lzStack.push(s);
        }
    }

    lzStack.top().pos = c.topPosition;
}

/** Remove from stack all segments whose end is reached.
 * Record changes if specified. Return number of removed. */
template <typename TNodeArray>
inline int LzTrie<TNodeArray>::removeUsedSegments() {
    int removed = 0;
    if (diag) cout<<"RMV;";
    while (lzStack.empty() == false) {
        ArraySegment s = lzStack.top();
        assert(s.pos <= s.end + 1);
        // we're outsize of the top segment, pop
        if (s.pos == s.end + 1) {
            if (recordChanges) lzStackPopped.push(s);
            lzStack.pop();
            removed++;
            if (lzStack.empty() == false)
                (lzStack.top()).pos++;
        }
        else break;
    }

    return removed;
}

/** If a position on top of the stack is a pointer, push the pointed
 * segment and repeat until non pointer is at the top. Return number of removed. */
template <typename TNodeArray>
inline int LzTrie<TNodeArray>::followPointers() {
    int pushed = 0;
    if (diag) cout<<"FLW;";
    while (true) {
        TNodeConst n = nodes[lzStack.top().pos];
        if (n.isPointer() == false) break;
        ArraySegment s;
        // closed pointer
        if (n.getSymbol() == 0) {
            s.pos = n.getSibling();
            s.end = N;
            // it must be the first pointer in a chain of pointers,
            // otherwise it is a bug
//            if (pushed != 0) {
//                cout << "pos: " << s.pos << endl;
//                cout << nodeArraySuffShortToString(nodes, s.pos) << endl;
//                for (int i = 0; i <= pushed; ++i) {
//                    ArraySegment s = lzStack.top(); lzStack.pop();
//                    cout << "pos: " << s.pos << endl;
//                    cout << nodeArraySuffShortToString(nodes, s.pos) << endl;
//                }
//                assert(pushed == 0);
//            }

        }
        // normal pointer
        else {
            s.pos = n.getSibling();
            s.end =  n.getSibling() + n.getSymbol();
        }

        lzStack.push(s);
        pushed++;
    }

    return pushed;
}

/** Print the stack state and the node to which top points. */
template <typename TNodeArray>
void LzTrie<TNodeArray>::printStack() {
    stack<ArraySegment> stackCopy = lzStack;
    ArraySegment last =  stackCopy.top();
    while (stackCopy.empty() == false) {
        ArraySegment top = stackCopy.top();
        cout << top.pos << " " << top.end << "| ";
        stackCopy.pop();
    }
    cout << nodeToString(nodes[last.pos]) <<"|" << nodeToString(nodes[last.pos+1]) << endl;
}

/** Clear stack data and bookkeeping. */
template <typename TNodeArray>
void LzTrie<TNodeArray>::clearStack() {
    // lzStack can be-non empty because it is not cleared after searchWord()
    // since the method must record stack position to continue DFS (listing of prefixes)
    // from the position of the word
    while (lzStack.empty() == false) lzStack.pop();
    // these stack should be empty since they are used only in DFS where
    // all stack operations are reverted upon finishing node traversal
    assert(lzStackPopped.empty());
    assert(lzStackChanges.empty());
}

/** Find the maximal prefix of the word that exists in the Trie.
 * When prefix not in the trie or the end of word is reached, stop and
 * leave stack unchanged so that Trie traversal can continue. */
template <typename TNodeArray>
void LzTrie<TNodeArray>::searchWord(typename TNodeArray::Symbol const * word) {
    recordChanges = false;

    wordFound = false;
    wordEnded = false;
    // TODO zahtjev na tip
    // push the first segment representing the whole array
    ArraySegment t(0, nodes.getSize() - 1);
    lzStack.push(t);

    // traverse the trie until end of the word is reached
    while(*word != zeroSymbol<TSymbol>()) {
        t = lzStack.top();
        TNodeConst n = nodes[t.pos];
        assert(n.isPointer() == false);

        if (*word == n.getSymbol()) {
            word++;
            if (*word == zeroSymbol<TSymbol>()) { // we're at the end of word
                wordEnded = true;
                if (n.getEow()) wordFound = true;
                break;
            }
            // if there's no word continuation in current branch, end search
            if (n.getCow() == false) break;
            // else, continue to the next char
            moveToNode(1);
        }
        else { // symbols don't match, attempt to branch off
            // end of branch, end search
            if (n.getSibling() == 0) break;
            //TODO zahtjev na tip TSymbol -> TIndex
            moveToNode(n.getSibling());
        }
    }
}

/** Search for a word in the LzTrie. Fast method that uses
 * inline stack-array. */
template <typename TNodeArray> template<class Titerator>
bool LzTrie<TNodeArray>::compressedContains(Titerator symbols) const {
    bool contains = false;
    // set inital capacity
    unsigned long scapacity = 50;
    //TODO ako ArraySegment dealocira memoriju u
    // destruktoru, dali ce ih free() pozivati?
    ArraySegment* stack = (ArraySegment *)malloc(scapacity * sizeof(ArraySegment));
    // number of els on the stack
    unsigned long ssize = 0;
    // TODO zahtjev na tip
    // push the first segment representing the whole array
    stack[ssize].set(0, nodes.getSize() - 1); ssize++;

    TIndex pos = stack[ssize-1].pos;
    while (*symbols != zeroSymbol<TSymbol>()) {
        TNodeConst n = this->nodes[pos];
        if (n.isPointer()) {
            // resize stack if needed
            if (ssize == scapacity) {
                scapacity *= 2;
                stack = (ArraySegment *)realloc(stack, scapacity * sizeof(ArraySegment));
            }
            //TODO zahtjev na +(TIndex,TSymbol)
            // push the segment
            stack[ssize].set( n.getSibling(), n.getSibling() + n.getSymbol() );
            ssize++;
            // continue at the segment beginning
            pos = stack[ssize-1].pos;
            continue;
        }
        else {
            if (*symbols == n.getSymbol()) {
                symbols++;
                // check if we're at the end of word
                if (*symbols == zeroSymbol<TSymbol>()) {
                    if (n.getEow()) contains = true;
                    break;
                }

                // if there's no word continuation in current branch, end search
                if (n.getCow() == false) break;

                // else, continue to the next char in topmost segment
                (stack[ssize-1].pos)++;

            }
            // symbols don't match, attempt to branch off
            else {
                // end of branch, end search
                if (n.getSibling() == 0) break;
                //TODO zahtjev na tip
                stack[ssize-1].pos += n.getSibling();
            }
        }

        // pop the stacked segments whose end is reached
        bool pop = false;
        while (ssize) {
            ArraySegment s = stack[ssize-1];
            assert(s.pos <= s.end + 1);
            // we're outsize of the top segment, pop
            if (s.pos == s.end + 1) {
                ssize--;
                pop = true;
                if (ssize) {
                    assert( nodes[stack[ssize-1].pos].isPointer() );
                    // increase the position at (new) top level
                    (stack[ssize-1].pos)++;
                }
            }
            // else, there are more elements to inspect at this level
            else break;
        }
        // topmost segment (entire array) ended, end search
        if (ssize == 0) break;
        // continue search at that position
        pos = stack[ssize-1].pos;
    }

    free(stack);
    return contains;
}

#endif	/* LZTRIE_H */
