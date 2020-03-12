#ifndef MAPFUNCTIONCACHE_H
#define MAPFUNCTIONCACHE_H

#include "IFunctionCache.h"

#include "stddef.h"
#include <map>
#include <list>

using namespace std;

template <typename TDomain, typename TCodomain>
class MapFunctionCache : public IFunctionCache<TDomain, TCodomain> {
public:
    typedef TDomain TDom;
    typedef TCodomain TCod;

    MapFunctionCache();
    MapFunctionCache(size_t size);

    void add(TDomain k, TCodomain v);
    bool contains(TDomain k);
    TCodomain fetch(TDomain k);
    void setSize(size_t size);
    void clear();

private:
    // add key->value mapping to cache
    void addEntry(TDomain k, TCodomain v);
    // remove key->value mapping from cache
    void popEntry();

    map<TDomain, TCodomain> cacheMap;
    list<TDomain> keyLifo;
    size_t maxSize, size;

    static const size_t DEFAULT_SIZE = 10;

};

template <typename TDomain, typename TCodomain>
MapFunctionCache<TDomain, TCodomain>::MapFunctionCache(): maxSize(size), size(DEFAULT_SIZE) { }

template <typename TDomain, typename TCodomain>
MapFunctionCache<TDomain, TCodomain>::MapFunctionCache(size_t size): maxSize(size), size(0) { }

template <typename TDomain, typename TCodomain>
TCodomain MapFunctionCache<TDomain, TCodomain>::fetch(TDomain k) {
    return cacheMap[k];
}

template <typename TDomain, typename TCodomain>
bool MapFunctionCache<TDomain, TCodomain>::contains(TDomain k) {
    return cacheMap.find(k) != cacheMap.end();
}

template <typename TDomain, typename TCodomain>
void MapFunctionCache<TDomain, TCodomain>::add(TDomain k, TCodomain v) {
    if (contains(k)) {
        // TODO: ?check value before update
        // TODO: re-prioritize element (cache refresh strategy)
        cacheMap[k] = v;
        return;
    }
    if (size == maxSize) popEntry();
    addEntry(k, v);
}

template <typename TDomain, typename TCodomain>
void MapFunctionCache<TDomain, TCodomain>::addEntry(TDomain k, TCodomain v) {
    cacheMap[k] = v;
    keyLifo.push_back(k);
    size++;
}

template <typename TDomain, typename TCodomain>
void MapFunctionCache<TDomain, TCodomain>::popEntry() {
    TDomain rk = keyLifo.front();
    keyLifo.pop_front();
    cacheMap.erase(rk);
    size--;
}

template <typename TDomain, typename TCodomain>
void MapFunctionCache<TDomain, TCodomain>::clear() {
    cacheMap.clear();
    keyLifo.clear();
    size = 0;
}

template <typename TDomain, typename TCodomain>
void MapFunctionCache<TDomain, TCodomain>::setSize(size_t size) {
    if (size <= 0) return;
    clear();
    maxSize = size;
}

#endif /* MAPFUNCTIONCACHE_H */
