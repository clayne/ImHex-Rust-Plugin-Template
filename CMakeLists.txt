cmake_minimum_required(VERSION 3.16)

# Change this to the name of your plugin #
project(example_rust)

# ---- No need to change anything from here downwards unless you know what you're doing ---- #

# Include ImHex cmake module #
set(IMHEX_BASE_FOLDER ${CMAKE_CURRENT_SOURCE_DIR}/ImHex)
set(CMAKE_MODULE_PATH "${IMHEX_BASE_FOLDER}/cmake/modules")
include("${IMHEX_BASE_FOLDER}/cmake/build_helpers.cmake")

# Include ImHex
add_subdirectory(${IMHEX_BASE_FOLDER})
addDefines()

# Create OS and Architecture defines #
detectOS()
detectArch()

# Detect build type and pass it on to cargo #
if (CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(CARGO_CMD ${RUST_PATH}cargo build)
    set(TARGET_DIR "debug")
else ()
    set(CARGO_CMD ${RUST_PATH}cargo build --release)
    set(TARGET_DIR "release")
endif ()

if (WIN32)
    set(PLUGIN_OUTPUT_PATH "${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/${PROJECT_NAME}${CMAKE_SHARED_LIBRARY_SUFFIX}")
else ()
    set(PLUGIN_OUTPUT_PATH "${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/${CMAKE_SHARED_LIBRARY_PREFIX}${PROJECT_NAME}${CMAKE_SHARED_LIBRARY_SUFFIX}")
endif ()

get_target_property(LIBIMHEX_SOURCE_DIRECTORY libimhex SOURCE_DIR)

# Call into cargo to compile rust plugin #
add_custom_target(${PROJECT_NAME} ALL
        COMMAND ${CMAKE_COMMAND} -E env
            CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR}
            LIBIMHEX_SOURCE_DIRECTORY=${LIBIMHEX_SOURCE_DIRECTORY}
            LIBIMHEX_OUTPUT_DIRECTORY=$<TARGET_FILE_DIR:libimhex>
            CXX_COMPILER=${CMAKE_CXX_COMPILER}
            IMHEX_VERSION_STRING=${IMHEX_VERSION_STRING}
            ${CARGO_CMD}
        COMMAND ${CMAKE_COMMAND} -E copy ${PLUGIN_OUTPUT_PATH} "${CMAKE_CURRENT_BINARY_DIR}/plugins/${PROJECT_NAME}.hexplug"
        WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
        VERBATIM
        USES_TERMINAL
)

set_target_properties(${PROJECT_NAME} PROPERTIES LOCATION ${CMAKE_CURRENT_BINARY_DIR})
set_target_properties(${PROJECT_NAME} PROPERTIES RUST_PROJECT 1)
set_target_properties(${PROJECT_NAME} PROPERTIES LINKER_LANGUAGE Rust)

add_compile_definitions(IMHEX_PLUGIN_NAME=${PROJECT_NAME})

set_target_properties(main PROPERTIES RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR})
set_target_properties(libimhex PROPERTIES RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR})
add_dependencies(imhex_all ${PROJECT_NAME})
set(PLUGINS ${PLUGINS} ${PROJECT_NAME})
foreach (plugin IN LISTS PLUGINS)
    set_target_properties(${plugin} PROPERTIES RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/plugins)
endforeach ()

set(CMAKE_SHARED_LIBRARY_PREFIX "")
set(CMAKE_SHARED_LIBRARY_SUFFIX ".hexplug")

set_target_properties(${PROJECT_NAME} PROPERTIES POSITION_INDEPENDENT_CODE ON)
setupCompilerWarnings(${PROJECT_NAME})