/* 
 * Interface for classes implementing character array.
 */

#ifndef ICHARARRAY_H
#define ICHARARRAY_H

#include <string>
#include "ISerializable.h"

using namespace std;

template <typename TCharLike>
class ICharArray : public ISerializable {
public:        
    /**
     *  Subclass must implement a public default constructor
     */
    
    // ICharArray();
    
    /** Fetch i-th element as a mutable reference. */
    virtual TCharLike operator[](size_t i) = 0;
    /** Allocate specified number of chars, possibly invalidating existing data. */
    virtual bool allocate(size_t size) = 0;
    /** Resize to specified number of chars, keeping existing data intact
     * up to the smaller of two sizes (existing and new). */
    virtual bool resize(size_t size) = 0;
    /** Free all memory. */
    virtual void freeMemory() = 0;
//    /** Persist array data to a file or folder. */
//    virtual bool persist(string f) = 0;
//   /** Load array data from a file or folder. */
//    virtual bool load(string f) = 0;
    
// TODO add get/set chars methods to interface?
    
    virtual ~ICharArray() {}; // just in case    
    
};

#endif /* ICHARARRAY_H */

