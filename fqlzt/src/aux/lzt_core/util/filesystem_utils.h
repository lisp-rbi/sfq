#ifndef FILESYSTEM_UTILS_H
#define FILESYSTEM_UTILS_H

#include <string>
#include <stdio.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <unistd.h>
#include <ftw.h>

using namespace std;

/** All the info returned by sys/ operations. */
struct InfoAndResult {
    InfoAndResult(struct stat inf, int res): info(inf), result(res) {};    
    struct stat info;
    int result;
};

static InfoAndResult file_info(string fname);
bool file_accessible(string fname);
bool file_is_regular(string fname);
bool file_is_directory(string fname);

/**
 * Helper function supporting data structure persistence 'protocol'. 
 * If f is a folder, create a filename by appending fname to path and return.
 * If f is a file, ignore fname and return f.
 * If either folder or file are nonexistent or non accessible, return "".
 */
string accessible_filename(string f, string fname);

/** Recursively remove all directory contents and the directory. */
bool remove_directory(string dname);

#endif /* FILESYSTEM_UTILS_H */

