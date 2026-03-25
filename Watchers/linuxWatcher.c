//
// Created by Jeevaanandh Ilayaraja on 25/03/26.
//


#include <stdio.h>
#include <string.h>
#include <dirent.h>
#include<sys/inotify.h>
#include <unistd.h>
#include<limits.h>

#include "builder.h"

#define BUFSIZE (1024 * (sizeof(struct inotify_event) + 16))
int fd;
char buf[BUFSIZE];

typedef struct {
    int wd;
    char fullPath[PATH_MAX];
} Watch;

int directories=0;
Watch WatchMap[1024];

int res=0;

int addWatch(char *path) {
    int wd= inotify_add_watch(
                    fd,
                    path,
                    IN_CLOSE_WRITE | IN_MOVED_TO | IN_MOVED_FROM | IN_CREATE   // Anytime one of these are triggered, a stuct is returned.
            );

    WatchMap[directories].wd= wd;
    strcpy(WatchMap[directories].fullPath, path);
    directories++;

    return 0;

}

char *getpath(int wd) {
    for (int i=0; i<directories; i++) {
        if (WatchMap[i].wd == wd) {
            return WatchMap[i].fullPath;

        }
    }

    return NULL;
}

void parse(char *path, int flag) {
    struct dirent *entry;
    DIR *dir= opendir(path);

    if (flag==0) {
        res= addWatch(path);
    }

    while ((entry= readdir(dir)) != NULL) {
        if (entry->d_name[0]=='.') {
            continue;
        }

        //If it is a directory watch it.
        if (entry->d_type == DT_DIR) {
            char fullpath[1024];
            snprintf(fullpath, sizeof(fullpath), "%s/%s", path, entry->d_name);
            res= addWatch(fullpath);

            parse(fullpath, flag+1);

        }

    }
}



//Modify the name of this function.
/*
	This function should take:
		1. Project folder path
		2. Build command
		3. Run command

	This is the function the CLI will call
*/
int watcher(char *projectFolder) {
    fd = inotify_init();

    parse(projectFolder, 0);

	//THis is the initail call to start the server.
    start_server(projectFolder);

    while(1) {
        //This waits until something is added to the buffer
        //The buffer stores the address to the structs.
        int len= read(fd, buf, BUFSIZE);
        int i=0;

        while (i<len) {
            struct inotify_event *event= (struct inotify_event *) &buf[i];

            if (event->len == 0) goto incr;
            if (event->name[0] == '.')    goto incr;  // hidden files
            if (strstr(event->name, "~"))    goto incr;
            if (strstr(evne->name, "/target")) goto incr;



            if ((event->mask & IN_CREATE) || (event->mask & IN_MOVED_TO)) {
                if (event->mask & IN_ISDIR) {
                    char *directoryPath= getpath(event->wd);
                    char fullpath[PATH_MAX];
                    if (!directoryPath) goto incr;
                    snprintf(fullpath, sizeof(fullpath), "%s/%s", directoryPath, event->name);

                    res= addWatch(fullpath);

                    printf("Directory modified: %s/%s\n", directoryPath, event->name);

                    start_server(projectFolder);
                }

                else {
                    goto incr;
                }
            }

            if (event->mask & IN_MOVED_FROM) {
                printf("Deleted: %s\n", event->name);
                start_server(projectFolder);
                goto incr;
            }



            if (!(strstr(event->name, ".java")) || !(strstr(event->name, ".xml")) ) goto incr;

            if (event->mask & IN_CLOSE_WRITE) {
                printf("File modified: %s\n", event->name);
                start_server(projectFolder);
            }






            incr:
            i+= sizeof(struct inotify_event)+event->len;


        }
    }

}







