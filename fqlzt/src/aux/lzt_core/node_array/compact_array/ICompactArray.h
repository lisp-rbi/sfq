/* 
 * Abstract interface classes compact node array.
 * TODO make this an interface for generic node array?
 */

#ifndef ICOMPACT_ARRAY
#define ICOMPACT_ARRAY

using namespace std;

template <typename TSymbol, typename TIndex, typename TNode>
class ICompactArray {
public:        

    typedef TSymbol Symbol;
    typedef TIndex Index;
    typedef TNode Node;
    
    virtual ~ICompactArray() {}; // just in case    
    
    virtual TNode operator[](TIndex i) = 0;
    virtual TIndex getSize() const = 0;
    virtual bool isEnumerated() const = 0;    
    virtual void setCache(size_t cacheSize) = 0;
    
};

#endif /* ICOMPACT_ARRAY */

