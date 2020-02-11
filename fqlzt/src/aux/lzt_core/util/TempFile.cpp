#include "TempFile.h"
#include "utils.h"
#include <cstdio>
#include <sys/types.h>
#include <sys/stat.h>
#include <unistd.h>

#include <stdio.h>
#include <ftw.h>
#include <unistd.h>

/* RECURSIVE DIRECTORY REMOVAL SOLUTION */
int unlink(const char *path, const struct stat *sb, int tflag, struct FTW *buff) {
    int res = remove(path);
    if (res) perror(path);
    return res;
}
int remove_recursive(char *path) {
    return nftw(path, unlink, 64, FTW_DEPTH | FTW_PHYS);
}
/* RECURSIVE DIRECTORY REMOVAL SOLUTION */

TempFile::TempFile(bool folder): isFolder(folder) {
    if (isFolder) name = "tmpfolder_";
    else name = "tmpfile_";
    name += getRandomString();
    if (isFolder) mkdir(name.c_str(), 0777);    
    else {
        file = fopen(name.c_str(), "w+");
        fclose(file);
    }
    //printf("%s\n", name.c_str());
}

TempFile::~TempFile() {
    //fclose(file);
    if (isFolder) remove_recursive(name.c_str());            
    else remove(name.c_str());
}

const char * TempFile::getName() {
    return name.c_str();
}