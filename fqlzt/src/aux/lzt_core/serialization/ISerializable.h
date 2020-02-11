
#ifndef ISERIALIZABLE_H
#define ISERIALIZABLE_H

#include <string>

using namespace std;

/* 
 * Interface for classes implementing serialization interface and protocol.
 * TODO document protocol.
 */
class ISerializable {
public:        
    
    /** Persist object data to a file or folder. */
    virtual bool persist(string f) = 0;
   /** Load object data from a file or folder. */
    virtual bool load(string f) = 0;    
    /** Write object data to a stream. Do not open/close the stream. */
    virtual void writeToStream(ostream& stream) = 0;
    /** Read object data from a stream. Do not open/close the stream. */
    virtual void readFromStream(istream& stream) = 0;
    
    virtual ~ISerializable() {}; // just in case    
    
};

#endif /* ISERIALIZABLE_H */

