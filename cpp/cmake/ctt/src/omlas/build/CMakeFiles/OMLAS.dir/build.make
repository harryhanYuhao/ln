# CMAKE generated file: DO NOT EDIT!
# Generated by "Unix Makefiles" Generator, CMake Version 3.25

# Delete rule output on recipe failure.
.DELETE_ON_ERROR:

#=============================================================================
# Special targets provided by cmake.

# Disable implicit rules so canonical targets will work.
.SUFFIXES:

# Disable VCS-based implicit rules.
% : %,v

# Disable VCS-based implicit rules.
% : RCS/%

# Disable VCS-based implicit rules.
% : RCS/%,v

# Disable VCS-based implicit rules.
% : SCCS/s.%

# Disable VCS-based implicit rules.
% : s.%

.SUFFIXES: .hpux_make_needs_suffix_list

# Command-line flag to silence nested $(MAKE).
$(VERBOSE)MAKESILENT = -s

#Suppress display of executed commands.
$(VERBOSE).SILENT:

# A target that is always out of date.
cmake_force:
.PHONY : cmake_force

#=============================================================================
# Set environment variables for the build.

# The shell in which to execute make rules.
SHELL = /bin/sh

# The CMake executable.
CMAKE_COMMAND = /snap/cmake/1210/bin/cmake

# The command to remove a file.
RM = /snap/cmake/1210/bin/cmake -E rm -f

# Escaping for special characters.
EQUALS = =

# The top-level source directory on which CMake was run.
CMAKE_SOURCE_DIR = /home/harryyuhaohan/study/coding/ln/cpp/cmake/ctt/src/omlas

# The top-level build directory on which CMake was run.
CMAKE_BINARY_DIR = /home/harryyuhaohan/study/coding/ln/cpp/cmake/ctt/src/omlas/build

# Include any dependencies generated for this target.
include CMakeFiles/OMLAS.dir/depend.make
# Include any dependencies generated by the compiler for this target.
include CMakeFiles/OMLAS.dir/compiler_depend.make

# Include the progress variables for this target.
include CMakeFiles/OMLAS.dir/progress.make

# Include the compile flags for this target's objects.
include CMakeFiles/OMLAS.dir/flags.make

CMakeFiles/OMLAS.dir/main.cpp.o: CMakeFiles/OMLAS.dir/flags.make
CMakeFiles/OMLAS.dir/main.cpp.o: /home/harryyuhaohan/study/coding/ln/cpp/cmake/ctt/src/omlas/main.cpp
CMakeFiles/OMLAS.dir/main.cpp.o: CMakeFiles/OMLAS.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/home/harryyuhaohan/study/coding/ln/cpp/cmake/ctt/src/omlas/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_1) "Building CXX object CMakeFiles/OMLAS.dir/main.cpp.o"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -MD -MT CMakeFiles/OMLAS.dir/main.cpp.o -MF CMakeFiles/OMLAS.dir/main.cpp.o.d -o CMakeFiles/OMLAS.dir/main.cpp.o -c /home/harryyuhaohan/study/coding/ln/cpp/cmake/ctt/src/omlas/main.cpp

CMakeFiles/OMLAS.dir/main.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing CXX source to CMakeFiles/OMLAS.dir/main.cpp.i"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E /home/harryyuhaohan/study/coding/ln/cpp/cmake/ctt/src/omlas/main.cpp > CMakeFiles/OMLAS.dir/main.cpp.i

CMakeFiles/OMLAS.dir/main.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling CXX source to assembly CMakeFiles/OMLAS.dir/main.cpp.s"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S /home/harryyuhaohan/study/coding/ln/cpp/cmake/ctt/src/omlas/main.cpp -o CMakeFiles/OMLAS.dir/main.cpp.s

# Object files for target OMLAS
OMLAS_OBJECTS = \
"CMakeFiles/OMLAS.dir/main.cpp.o"

# External object files for target OMLAS
OMLAS_EXTERNAL_OBJECTS =

OMLAS: CMakeFiles/OMLAS.dir/main.cpp.o
OMLAS: CMakeFiles/OMLAS.dir/build.make
OMLAS: CMakeFiles/OMLAS.dir/link.txt
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --bold --progress-dir=/home/harryyuhaohan/study/coding/ln/cpp/cmake/ctt/src/omlas/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_2) "Linking CXX executable OMLAS"
	$(CMAKE_COMMAND) -E cmake_link_script CMakeFiles/OMLAS.dir/link.txt --verbose=$(VERBOSE)

# Rule to build all files generated by this target.
CMakeFiles/OMLAS.dir/build: OMLAS
.PHONY : CMakeFiles/OMLAS.dir/build

CMakeFiles/OMLAS.dir/clean:
	$(CMAKE_COMMAND) -P CMakeFiles/OMLAS.dir/cmake_clean.cmake
.PHONY : CMakeFiles/OMLAS.dir/clean

CMakeFiles/OMLAS.dir/depend:
	cd /home/harryyuhaohan/study/coding/ln/cpp/cmake/ctt/src/omlas/build && $(CMAKE_COMMAND) -E cmake_depends "Unix Makefiles" /home/harryyuhaohan/study/coding/ln/cpp/cmake/ctt/src/omlas /home/harryyuhaohan/study/coding/ln/cpp/cmake/ctt/src/omlas /home/harryyuhaohan/study/coding/ln/cpp/cmake/ctt/src/omlas/build /home/harryyuhaohan/study/coding/ln/cpp/cmake/ctt/src/omlas/build /home/harryyuhaohan/study/coding/ln/cpp/cmake/ctt/src/omlas/build/CMakeFiles/OMLAS.dir/DependInfo.cmake --color=$(COLOR)
.PHONY : CMakeFiles/OMLAS.dir/depend
