#include <stdio.h>
#include <sys/syscall.h>
#include <linux/memfd.h>
#include <unistd.h>
#include <err.h>
#include <string.h>

int main() {
    char *hello = "#!/bin/bash\necho \"hello fileless\"";
    int fd;
    if ((fd = syscall(SYS_memfd_create, "test", 0)) == -1)
        err(1, "memfd_create");
    write(fd, hello, strlen(hello));
    if (fexecve(fd, (char *[]){"script", NULL}, (char*[]){NULL}) == -1   )
        perror("fexecve");
}
