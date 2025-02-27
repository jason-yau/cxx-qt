<!--
SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# QObjects in Rust

> The right tool for the right job.

> If you only have a hammer, every problem looks like a nail.

> Don't bring a knife to a gun fight.

There are many bits of advice like that.
With CXX-Qt, we aim to make it possible to use the right tool for each of the many jobs necessary to build a modern GUI application.

So what is in our toolbox for a typical Qt application?
- QML - A declarative, flexible, dynamically-typed, interpreted language that is purpose built to define reactive and beautiful GUI layouts and widgets with quick iteration speed.
- C++ - The traditional back-end of Qt - A fast, low-level language with a strong type system. C++ offers a rich ecosystem, many Qt-specific libraries and bare-metal performance. The cost when using C++ is that it is slow to develop, very error-prone and can easily lead to memory-issues, which can instantly crash your application and cause security issues.

Notably absent then is a back-end language that allows us to get rid of the issues C++ has and provides us with a safe way to write fast back-end code.
This of course is where Rust comes in.
Whilst Rust doesn't have quite as rich of an ecosystem, it is typically faster to develop than C++, with easy dependency management, and most importantly, safe memory access.
Therefore it is an ideal candidate to replace C++ for writing the back-end business-logic code that feeds the GUI with data.

However, C++ as well as QML still have their place in Qt applications.
For that reason Rust, C++, and QML should all be able to be used to complement each other.
CXX-Qt aims to make it easy to integrate all three languages easily with each other, through the use of Qt's [meta object system](https://doc.qt.io/qt-5/metaobjects.html).

Qt's design is inherently object-oriented, which is true both for C++ and QML as well.
Therefore, in order to integrate well with Qt, Rust needs to be able to extend the Qt object system with its own QObject subclasses and instances.
This is exactly what CXX-Qt allows you to do.

As Rust doesn't offer classes with inheritance and polymorphism, CXX-Qt uses macros when defining new QObject subclasses.

These CXX-Qt modules consist of multiple parts:
- A `Data` struct
    - Defines which Properties will be in the QObject subclass.
    - Needs to implement the `Default` trait.
    - This data will live as properties in the C++ subclass that is generated by CXX-Qt.
- A struct marked with a `#[cxx_qt::qobject]` macro
    - Defines the C++ QObject subclass name
    - A normal Rust struct.
    - One struct instance is created per class instance.
    - Contains any Rust-only data.
    - Needs to implement the `Default` trait.
- The `impl` of the `#[cxx_qt::qobject]` marked struct (optional):
    - Contains any Rust code.
    - Functions marked with `#[qinvokable]` will be callable from QML and C++.
- The `Signal` enum
    - A normal Rust enum.
    - Defines signals that are added to the QObject class

CXX-Qt will then expand this Rust module into two separate parts:
- A C++ subclass of QObject with the same name as the module
- The `#[cxx_qt::qobject]` marked Rust struct

<div style="background-color: white; padding: 1rem; text-align: center;">

![Overview of CXX-Qt module generation](../images/overview_abstract.svg)

</div>

CXX-Qt also generates the code needed for interaction of the C++ QObject subclass and the `#[cxx_qt::qobject]` marked struct using the [CXX library](https://cxx.rs/).
For more details, see the [Concepts: Bridge](../concepts/bridge.md) page.
Additionally, CXX-Qt wraps some Qt types for us, so they can be used easily by the Rust side.
See the [Concepts: Qt Types](../concepts/types.md) page for the list of available types.

The important take away here is the duality of any subclass generated by CXX-Qt.
These classes are made up of the actual QObject subclass instance that exists purely on the C++ side, as well as an instance of the `#[cxx_qt::qobject]` marked struct.
The lifetime and GUI data is therefore managed by the QObject instance on the C++ side.
Typically this will be instantiated by QML and the lifetime will be directly associated with the corresponding QML widget.
Any properties declared in the `Data` struct will be stored as a member of the C++ QObject.

However, the generated QObject subclass will defer to the `#[cxx_qt::qobject]` marked struct for any behavior, which is then defined in Rust.
The `#[cxx_qt::qobject]` marked struct can expose additional functionality with functions marked as `#[qinvokable]`, which will generate a function on the C++ side that will directly call the appropriate Rust method.
These Rust methods can take a reference to the members of the C++ object via a wrapper called `CppObj`, so the Rust code can modify them.

Now that we have taken a look the theory of it all, lets jump in and write [our first CXX-Qt module](./2-our-first-cxx-qt-module.md).

