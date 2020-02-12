#include "TempFile.h"

TempFile::TempFile(bool folder): isFolder(folder) {
    if (isFolder) name = "tmpfolder_";
    else name = "tmpfile_";
    name += getRandomString();
    if (isFolder) mkdir(name.c_str(), 0777);    
    else {
        file = fopen(name.c_str(), "w+");
        fclose(file);
    }    
}

TempFile::~TempFile() {   
    if (isFolder) remove_directory(name);            
    else remove(name.c_str());    
}

const char * TempFile::getName() {
    return name.c_str();
}