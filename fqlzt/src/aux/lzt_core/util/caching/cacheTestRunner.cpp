#include "cacheTestRunner.h"

void runCacheTests() {
    //simpleTest();    
    mapFuncCacheTests();
}

void simpleTest() {
    MapFunctionCache<size_t, char> cache(5);
    for (int i = 0; i < 10; ++i) {
        cache.contains(i);
        cache.add(i, i+1);
        cache.fetch(i);
    }
}

void mapFuncCacheTests() {
    CacheTester<MapFunctionCache<int, long> > tester;
    tester.intDomainBasicTest(10000);
    tester.intDomainLifoTest(10000);
}