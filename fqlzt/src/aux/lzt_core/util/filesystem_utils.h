#ifndef FILESYSTEM_UTILS_H
#define FILESYSTEM_UTILS_H

#include <string>
#include <iostream>
#include <stdio.h>
#include <stdlib.h>
#include <limits.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <unistd.h>
#include <ftw.h>
#include <fcntl.h>
#include <sys/sendfile.h>

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
/** Copy file source to location dest, return true upon success. */
bool copy_file(string fpathSource, string fpathDest);
/** Convert file path to absolute (full path from root) form, return "" on error. 
 * File must exist in order for this implementation to work. 
 * TODO - use an API method that works only with paths. */
string absolute_path(string fname);
/** Create empty file with the specified path. Return true on success */
bool create_file(string fpath);
/** Delete file with the specified path. Return true on success */
bool delete_file(string fpath);

#endif /* FILESYSTEM_UTILS_H */

