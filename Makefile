CC = gcc
CFLAGS = -Wall

TARGET = wfly
PREFIX = /usr/local/bin

# Detect OS
UNAME_S := $(shell uname -s)

# Default values
FRAMEWORKS =
SRC = CLI.c Watchers/builder.c

# OS-specific config
ifeq ($(UNAME_S), Darwin)
    SRC += Watchers/macWatcher.c
    FRAMEWORKS = -framework CoreServices -framework CoreFoundation
else ifeq ($(UNAME_S), Linux)
    SRC += Watchers/linuxWatcher.c
endif

OBJ = $(SRC:.c=.o)

all: $(TARGET)

$(TARGET): $(OBJ)
	$(CC) $(OBJ) -o $(TARGET) $(FRAMEWORKS)

%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@

install: $(TARGET)
	sudo cp $(TARGET) $(PREFIX)/$(TARGET)
	sudo chmod +x $(PREFIX)/$(TARGET)
	@echo "Installed $(TARGET) to $(PREFIX)"

uninstall:
	sudo rm -f $(PREFIX)/$(TARGET)
	@echo "$(TARGET) removed from $(PREFIX)"

clean:
	rm -f $(OBJ) $(TARGET)