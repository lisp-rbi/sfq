#include "filesystem_utils.h"

InfoAndResult file_info(string fname) {
    struct stat info;
    int res = stat(fname.c_str(), &info);
    return InfoAndResult(info, res);
}

bool file_accessible(string fname) {
    InfoAndResult ir = file_info(fname);
    return (ir.result == 0);
}

bool file_is_regular(string fname) {
    InfoAndResult ir = file_info(fname);
    if (ir.result != 0) return false;
    if (ir.info.st_mode & S_IFREG) return true;
    else return false;
}

bool file_is_directory(string fname) {
    InfoAndResult ir = file_info(fname);
    if (ir.result != 0) return false;
    if (ir.info.st_mode & S_IFDIR) return true;
    else return false;
}

string accessible_filename(string f, string fname) {
    if (file_accessible(f) == false) return "";    
    if (file_is_regular(f)) return f;
    else if (file_is_directory(f)) {
        size_t end = f.length()-1;
        // TODO this is unix-only
        if (f[end] == '/') return f + fname;
        else return f + "/" + fname;
    }
    else return "";    
}