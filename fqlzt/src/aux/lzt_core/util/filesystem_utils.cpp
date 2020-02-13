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

int unlink(const char *path, const struct stat *sb, int tflag, struct FTW *buff) {
    int res = remove(path);    
    return res;
}

bool remove_directory(string dname) {
    int res = nftw(dname.c_str(), unlink, 64, FTW_DEPTH | FTW_PHYS);
    return res == 0;
}

bool copy_file(string fnameSource, string fnameDest) { //(const char* source, const char* destination)
    int input, output;    
    if ((input = open(fnameSource.c_str(), O_RDONLY)) == -1) return false;    
    if ((output = creat(fnameDest.c_str(), 0660)) == -1)
    {
        close(input);
        return false;
    }
    off_t bytesCopied = 0;
    struct stat fileinfo = {0};
    fstat(input, &fileinfo);
    int result = sendfile(output, input, &bytesCopied, fileinfo.st_size);
    close(input);
    close(output);
    return result == fileinfo.st_size;
}