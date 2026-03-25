//
// Created by Jeevaanandh Ilayaraja on 25/03/26.
//

#include <stdio.h>
#include <getopt.h>
#include <unistd.h>
#include <signal.h>
#include "Watchers/builder.h"
#include "Watchers/macWatcher.h"


int main(int argc, char *argv[]) {
    signal(SIGINT, handle_sigint);
    signal(SIGCHLD, handle_sigchld);

    char *root = NULL;


    int opt;

    struct option long_options[] = {
        {"root",  required_argument, 0, 'r'},

        {0,0,0,0}
    };

    while ((opt = getopt_long(argc, argv, "", long_options, NULL)) != -1) {

        switch (opt) {
        case 'r':
            root = optarg;
            break;


        default:
            printf("Unknown option\n");
            return -1;
        }
    }



    watcher(root);
}