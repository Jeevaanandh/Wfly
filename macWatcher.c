//
// Created by Jeevaanandh Ilayaraja on 23/03/26.
//

#include<stdio.h>
#include<unistd.h>
#include<sys/event.h>
#include<sys/types.h>
#include<CoreServices/CoreServices.h>


void callback(
    ConstFSEventStreamRef streamRef,
    void *clientCallBackInfo,
    size_t numEvents,
    void *eventPaths,
    const FSEventStreamEventFlags flags[],
    const FSEventStreamEventId eventIds[]) {

    char **paths= eventPaths;

    printf("Change Occured\n");


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