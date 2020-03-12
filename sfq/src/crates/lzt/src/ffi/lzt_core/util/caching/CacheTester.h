#ifndef CACHETESTER_H
#define CACHETESTER_H

#include "debug/lzt_test.h"
#include <sstream>

using namespace std;

template <typename TCache>
class CacheTester {
    public: 
        
    typedef typename TCache::TDom TD;
    typedef typename TCache::TCod TC;
    
    // Tests in case where TDomain and TCodomain are integer types
    // basic add/contains/fetch tests
    void intDomainBasicTest(size_t max);
    // test cache with lifo replacement strategy
    void intDomainLifoTest(size_t max);
};

template <typename TCache>
void CacheTester<TCache>::intDomainBasicTest(size_t max) {
    TCache cache(max);
    TD i;
    for (i = 0; i < max; ++i) {
        cache.add(i, i+1);
        TEST_ASSERT(cache.contains(i));
        TEST_ASSERT(cache.fetch(i) == i+1);
    }
}

template <typename TCache>
void CacheTester<TCache>::intDomainLifoTest(size_t max) {
    size_t sz = max/10+2;
    TCache cache(sz);
    TD i;
    for (i = 0; i < max; ++i) {
        cache.add(i, i+1);
        TEST_ASSERT(cache.contains(i));
        TEST_ASSERT(cache.fetch(i) == i+1);
    }
    cache.clear();
    for (i = 0; i < max; ++i) {
        cache.add(i, i+1);
        TEST_ASSERT(cache.contains(i));        
        TEST_ASSERT(cache.fetch(i) == i+1);
        if (i >= sz) {
            ostringstream ss; 
            ss<<"i: "<<i<<", max: "<<max<<", sz: "<< sz;
            TEST_ASSERT_MESSAGE(cache.contains(i-sz) == false, ss.str());
            TEST_ASSERT_MESSAGE(cache.contains(i-sz+1), ss.str());
        }
    }    
}

#endif /* CACHETESTER_H */

