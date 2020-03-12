/* 
 * Interface for classes function cache.
 */

#ifndef IFUNCTIONCACHE_H
#define IFUNCTIONCACHE_H

#include <string>

using namespace std;

template <typename TDomain, typename TCodomain>
class IFunctionCache {
public:        
    
    virtual void add(TDomain k, TCodomain v) = 0;
    virtual bool contains(TDomain k) = 0;
    virtual TCodomain fetch(TDomain k) = 0;
    virtual void clear() = 0;
    virtual void setSize(size_t size) = 0;
    
    virtual ~IFunctionCache() {}; 
    
};

#endif /* IFUNCTIONCACHE_H */

