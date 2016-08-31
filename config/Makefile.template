TARGET_NAME = __PAC_NAME__
CXX = __COMPILER__
CXXFLAGS = __ARGS__

LDFLAGS = __LIB_DIRS__
LIBS = __LIBS__
INCLUDE = __INCLUDES__

SRCDIR = __SRC_DIR__
SRC = $(wildcard $(SRCDIR)/*.cpp)

BUILD_DIR = __DEST_DIR__
TARGET = $(BUILD_DIR)/$(TARGET_NAME)

OBJ = $(addprefix $(BUILD_DIR)/obj/, $(notdir $(SRC:.cpp=.o)))
DEPEND = $(OBJ:.o=.d)

all: $(TARGET)

-include $(DEPEND)

$(TARGET): $(OBJ)
	$(CXX) -o $@ $^ $(LDFLAGS) $(LIBS)


$(BUILD_DIR)/obj/%.o: $(SRCDIR)/%.cpp
	$(CXX) $(CXXFLAGS) $(INCLUDE)  -o $@ -c -MMD -MP $<


.PHONY: clean
clean:
	-rm -f $(OBJ) $(DEPEND) $(TARGET)

run: $(TARGET)
	$(TARGET)

