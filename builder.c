//
// Created by Jeevaanandh Ilayaraja on 24/03/26.
//
#include<stdio.h>
#include<string.h>
#include<stdlib.h>
#include<unistd.h>
#include<signal.h>

pid_t cid=-1;


int run_clean(char* rootPath) {
    pid_t pid= fork();
    if (pid == 0) {
        chdir(rootPath);

        execl("/bin/sh", "sh", "-c", "mvn clean package", NULL);

    }

    int status;
    waitpid(pid, &status, 0);

    if (WIFEXITED(status)){
        int code = WEXITSTATUS(status);

        if (code==0) {
            return 0;
        }

        else {
            return 1;
        }
    }

    return 1;

}

void run(char* rootPath) {
    cid=fork();
    if (cid == 0)
    {
        setpgid(0,0);
        chdir(rootPath);
        execl("/bin/sh", "sh", "-c", "mvn wildfly-jar:run", NULL);

    }


}

int kill_child() {
    if (cid == -1) {
        return 0;
    }

    killpg(cid, SIGKILL);
    waitpid(cid, NULL, 0);
    cid = -1;


}

//This is what The Watcher Calls.
void start_server(char* rootPath) {
    int res;

    res= run_clean(rootPath);

    if (res==0) {
        kill_child();

        run(rootPath);

    }

}