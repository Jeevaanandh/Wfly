CC = gcc
CFLAGS = -Wall

TARGET = wfly
SRC = CLI.c Watchers/macWatcher.c Watchers/builder.c
OBJ = $(SRC:.c=.o)

PREFIX = /usr/local/bin

# macOS frameworks
FRAMEWORKS = -framework CoreServices -framework CoreFoundation

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