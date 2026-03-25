//
// Created by Jeevaanandh Ilayaraja on 24/03/26.
//

#ifndef WILDFLYWATCHER_BUILDER_H
#define WILDFLYWATCHER_BUILDER_H

void start_server(char* rootPath);
void handle_sigchld(int sig);
void handle_sigint(int sig);

#endif //WILDFLYWATCHER_BUILDER_H