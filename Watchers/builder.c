//
// Created by Jeevaanandh Ilayaraja on 24/03/26.
//



//THIS CAN BE REUSED FOR LINUX TOO.



#include<stdio.h>
#include<string.h>
#include<stdlib.h>
#include<unistd.h>
#include<signal.h>


pid_t run_cid=-1;
pid_t build_cid=-1;
char *Path;

void run(char* rootPath);


//Remember: This is used t kill the child process that runs the server
//So, after we kill it, the parent should be notified, otherwise, the child will remain a zombie.
void kill_child(pid_t *pid) {
    if (*pid == -1) {
        return ;
    }

    printf("\n\n\nKilling...\n\n\n");

    killpg(*pid, SIGKILL);
    waitpid(*pid, NULL, 0);
    *pid = -1;



}


//This is the Handler for SIGCHLD ----- The OS calls this function when a child exits. (THIS IS TO PREVENT ZOMBIE AND ENSURE THAT run() IS CALLED AFTER THE BUILD IS DONE).
void handle_sigchld(int sig) {
    int status;
    pid_t pid;
    while ((pid = waitpid(-1, &status, WNOHANG)) > 0) {
        if (pid == build_cid) {
            build_cid= -1;
            kill_child(&run_cid);

            run(Path);
        }

    }

}


void handle_sigint(int sig) {
    printf("\n\n\nStopping wfly...\n\n\n");

    if (run_cid > 0) {
        killpg(run_cid, SIGTERM);
    }

    _exit(0);
}




//So, after this child exits ----- a SIGCHLD is raised and the run() function is called from there.
void run_clean(char* rootPath) {
    build_cid= fork();
    if (build_cid == 0) {
        setpgid(0,0);
        chdir(rootPath);

        execl("/bin/sh", "sh", "-c", "mvn clean package", NULL);

    }
}


void run(char* rootPath) {
    run_cid=fork();
    if (run_cid == 0)
    {
        setpgid(0,0);
        chdir(rootPath);
        execl("/bin/sh", "sh", "-c", "mvn wildfly:deploy", NULL);

    }


}




//This is what The Watcher Calls.
void start_server(char* rootPath) {

    Path= rootPath;

    kill_child(&build_cid);

    run_clean(rootPath);

}
