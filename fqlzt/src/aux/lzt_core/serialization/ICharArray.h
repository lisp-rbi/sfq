/* 
 * Interface for classes implementing character array.
 */

#ifndef ICHARARRAY_H
#define ICHARARRAY_H

class ICharArray {
public:        
    // ? Subclass must implement a public default constructor
    // ICharArray();
    virtual char& operator[](size_t i) = 0;
    virtual ~ICharArray() {};
    
    // ? ICharArray(const ICharArray& orig);
};

#endif /* ICHARARRAY_H */

