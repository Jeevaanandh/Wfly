//
// Created by Jeevaanandh Ilayaraja on 23/03/26.
//

#include<stdio.h>
#include<unistd.h>
#include<sys/event.h>
#include<sys/types.h>
#include<CoreServices/CoreServices.h>
#include<string.h>
#include<limits.h>
#include "builder.h"

/*

 callback() is called whenever there are queued notification after a time window (latency)

 The callback() function deals with what should be done whenever there is an event

*/

char path[MAXPATHLEN];

void callback(
    ConstFSEventStreamRef streamRef,
    void *clientCallBackInfo,
    size_t numEvents,
    void *eventPaths,
    const FSEventStreamEventFlags flags[],
    const FSEventStreamEventId eventIds[]) {

    char **paths= eventPaths;

    int flag=0;
    for(int i=0;i<numEvents;i++) {

        if (strstr(paths[i], "/target")!=NULL) {
            continue;
        }
        if (strstr(paths[i],".java") != NULL || strstr(paths[i],".xml") != NULL) {
            flag=1;
            break;
        }

    }

    if (flag==1) {
        //Call the function to run the commands here.
        printf("Change Occured\n");
        start_server(path);

    }



}


//Change the name of this function when using CLI
int main() {
    //Move this to the main() of the CLI when you start with the CLI
    signal(SIGCHLD, handle_sigint);

    strcpy(path, "/Users/JeevaanandhIlayaraja/Desktop/MicroProfileTesting/service-a");

    CFStringRef pathToWatch = CFStringCreateWithCString(
        NULL,
        path,
        kCFStringEncodingUTF8
    );

    CFArrayRef pathsToWatch= CFArrayCreate(NULL, (const void **)&pathToWatch, 1, NULL);

    void *callbackInfo= NULL;
    FSEventStreamRef stream;
    CFAbsoluteTime latency= 1.0;

    stream= FSEventStreamCreate(NULL,
        &callback,
        callbackInfo,
        pathsToWatch,
        kFSEventStreamEventIdSinceNow,
        latency,
        kFSEventStreamCreateFlagFileEvents
        );

    FSEventStreamScheduleWithRunLoop(stream, CFRunLoopGetCurrent(), kCFRunLoopDefaultMode);
    FSEventStreamStart(stream);
    CFRunLoopRun();

}