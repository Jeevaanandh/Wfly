//
// Created by Jeevaanandh Ilayaraja on 23/03/26.
//

#include<stdio.h>
#include<unistd.h>
#include<sys/event.h>
#include<sys/types.h>
#include<CoreServices/CoreServices.h>
#include<string.h>

/*

 callback() is called whenever there are queued notification after a time window (latency)

 The callback() function deals with what should be done whenever there is an event

*/

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
        if (strstr(paths[i],".txt") != NULL) {
            flag=1;
        }

    }

    if (flag==1) {
        printf("Change Occured\n");
    }



}


int main() {

    CFStringRef pathToWatch= CFSTR("./testdir");
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