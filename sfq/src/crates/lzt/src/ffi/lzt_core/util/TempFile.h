#ifndef TEMPFILE_H
#define	TEMPFILE_H

#include "utils.h"
#include "filesystem_utils.h"

#include <string>

using std::string;

/** Creates the file at construction and deletes it at destruction. */
class TempFile {
public:
    TempFile(bool isFolder=false);    
    virtual ~TempFile();   

    const char * getName();

private:
    bool isFolder;
    string name;
    FILE* file;

    TempFile(const TempFile& orig);

};

#endif	/* TEMPFILE_H */

