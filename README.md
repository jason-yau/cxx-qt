<!--
SPDX-FileCopyrightText: 2021-2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# CXX-Qt

CXX-Qt is a library that automatically generates code to transfer data between Rust and C++ through common interfaces
such as QObjects that can be exposed directly into QML. It relies on the cxx crate internally to achieve this and thus
it is recommended that any interactions with Qt that are not covered by the built-in code generators should be done
directly in C++ and connected to relevant Rust logic by writing additional cxx code. The CXX-Qt build system is based
on CMake, but is compatible with cxx on its own as well.

The examples folder contains an example application using the CXX-Qt crate and will be used for development and testing
purposes. The cxx-qt folder contains the source for the actual crate which contains a proc-macro. The cxx-qt-gen folder
contains the source for a crate which extracts and generates C++ and Rust source code. The cxx-qt-build folder contains
the source for a crate which provides helper functions to be used in a `build.rs` file.

Initially the projects in the examples folder will also serve as a template for new projects should use CXX-Qt.
In future we might improve upon this with a custom CMake module for instance.

## Getting Started

If you want to use CXX-Qt and see it in action visit our Getting Started guide in our Rust book [https://kdab.github.io/cxx-qt/book/getting-started/index.html](https://kdab.github.io/cxx-qt/book/getting-started/index.html).

Here we go through the steps of creating a Rust project and exposing to QML.

For more complex examples navigate to the [examples folder](./examples) where there are demonstrations of using threading, QQmlExtensionPlugin, and various other features.

Below is an example of Rust code that exposes a QObject with two properties and four invokable methods to Qt.

```rust
use serde::{Deserialize, Serialize};

// Represent the Data struct below with serde friendly types, so we can (de)serialize it
#[derive(Deserialize, Serialize)]
pub struct DataSerde {
    number: i32,
    string: String,
}

impl From<Data> for DataSerde {
    fn from(value: Data) -> DataSerde {
        DataSerde {
            number: value.number,
            string: value.string.to_string(),
        }
    }
}

const DEFAULT_STR: &str = r#"{"number": 1, "string": "Hello World!"}"#;

#[cxx_qt::bridge(namespace = "cxx_qt::my_object")]
mod ffi {
    use super::{DataSerde, DEFAULT_STR};

    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/include/qt_types.h");
        type QString = cxx_qt_lib::QString;
    }

    pub struct Data {
        pub number: i32,
        pub string: UniquePtr<QString>,
    }

    impl Default for Data {
        fn default() -> Self {
            let data_serde: DataSerde = serde_json::from_str(DEFAULT_STR).unwrap();
            data_serde.into()
        }
    }

    impl From<DataSerde> for Data {
        fn from(value: DataSerde) -> Data {
            Data {
                number: value.number,
                string: QString::from_str(&value.string),
            }
        }
    }

    #[cxx_qt::qobject]
    #[derive(Default)]
    pub struct MyObject;

    impl cxx_qt::QObject<MyObject> {
        #[qinvokable]
        pub fn increment(&self, cpp: &mut CppObj) {
            cpp.set_number(cpp.number() + 1);
        }

        #[qinvokable]
        pub fn reset(&self, cpp: &mut CppObj) {
            let data: DataSerde = serde_json::from_str(DEFAULT_STR).unwrap();
            cpp.grab_values_from_data(data.into());
        }

        #[qinvokable]
        pub fn serialize(&self, cpp: &mut CppObj) -> UniquePtr<QString> {
            let data = Data::from(cpp);
            let data_serde = DataSerde::from(data);
            let data_string = serde_json::to_string(&data_serde).unwrap();
            QString::from_str(&data_string)
        }

        #[qinvokable]
        pub fn grab_values(&self, cpp: &mut CppObj) {
            let string = r#"{"number": 2, "string": "Goodbye!"}"#;
            let data: DataSerde = serde_json::from_str(string).unwrap();
            cpp.grab_values_from_data(data.into());
        }
    }
}
```


## Contributing to CXX-Qt

### Building

Ensure that you have the following installed

  * C++ compiler
  * [clang-format](https://clang.llvm.org/docs/ClangFormat.html)
  * [CMake v3.16+](https://cmake.org/)
  * [Qt 5 or Qt 6 (experimental)](https://www.qt.io/)
  * [Rust](https://www.rust-lang.org/)
  * Linux 64-bit x86 - currently we only support Linux, but we plan on adding arm 64-bit, macOS, and Windows support in the future

### Compiling
In a CXX-Qt project, the build system is based on CMake, which uses Cargo under the hood.
Therefore, unlike a typical Rust project, CMake must be used to build CXX-Qt.

On Windows and macOS, CXX-Qt defaults to installing Qt from vcpkg. Prebuilt packages are
automatically downloaded from GitHub Packages (this will take several minutes the first time
you run CMake). If you already have Qt installed, you can disable this by adding
`-D VCPKG=OFF` to the CMake configure step (the first call to `cmake`).

CXX-Qt defaults to building with Qt6. If you want to build with Qt5 when both are installed,
or you want to tell vcpkg to use Qt5, add `-D QT_DEFAULT_MAJOR_VERSION=5` to the CMake
configure step.

```bash
mkdir build/
cd build/
cmake ../
cmake --build . -j$(nproc)
```

### Run the basic QML example

```bash
./build/examples/qml_minimal/example_qml_minimal
```

### Book

You can build the book using `mdbook serve` from the `book` folder, you should install `mdbook` and `mdbook-linkcheck` from cargo.

### Testing
Testing assumes that `cargo clippy` and `cargo fmt` are available, you may need to install these with `rustup component add clippy rustfmt`.

For testing the book, it assumes that `mdbook` and `mdbook-linkcheck` from cargo have been installed.

For license and memory testing, it assumes that you have [`reuse`](https://reuse.software/) installed (eg via `pip3 install reuse`) and [`valgrind`](https://valgrind.org/).

```bash
cd build/
ctest -j$(nproc)
```

## Licensing

CXX-Qt is Copyright (C) 2021, Klarälvdalens Datakonsult AB, and is available under
the terms of the [MIT](https://github.com/KDAB/cxx-qt/blob/main/LICENSES/MIT.txt)
or the [Apache-2.0](https://github.com/KDAB/cxx-qt/blob/main/LICENSES/Apache-2.0.txt)
licenses.

Contact KDAB at <info@kdab.com> to inquire about additional features or
services related to this project.

CXX-Qt includes these source files, also available under the terms of the MIT license:

* [doctest.h](https://github.com/onqtam/doctest) - the lightest feature-rich C++ single-header testing framework for unit tests and TDD (C) 2016-2021 Viktor Kirilov <vik.kirilov@gmail.com>

The following CMake source files are available under the BSD-3-Clause

* [cmake/CompilerCaching.cmake](./cmake/CompilerCaching.cmake) - a helper for using sccache

# About KDAB

CXX-Qt is supported and maintained by Klarälvdalens Datakonsult AB (KDAB).

The KDAB Group is the global No.1 software consultancy for Qt, C++ and
OpenGL applications across desktop, embedded and mobile platforms.

The KDAB Group provides consulting and mentoring for developing Qt applications
from scratch and in porting from all popular and legacy frameworks to Qt.
We continue to help develop parts of Qt and are one of the major contributors
to the Qt Project. We can give advanced or standard trainings anywhere
around the globe on Qt as well as C++, OpenGL, 3D and more.

Please visit https://www.kdab.com to meet the people who write code like this.
