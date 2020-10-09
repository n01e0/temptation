#include <stdio.h>
#include <sys/syscall.h>
#include <linux/memfd.h>
#include <unistd.h>
#include <err.h>
#include <string.h>

#define HELLO "#!/bin/bash\necho \"hello fileless\""

int main() {
    int fd;
    if ((fd = syscall(SYS_memfd_create, "test", 0)) == -1)
        err(1, "memfd_create");
    write(fd, HELLO, strlen(HELLO));
    if (fexecve(fd, (char *[]){"script", NULL}, (char*[]){NULL}) == -1   )
        err(1, "fexecve");
}
