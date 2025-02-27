// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
import QtQuick 2.12
import QtTest 1.12

import com.kdab.cxx_qt.demo 1.0

TestCase {
    name: "MyObjectTests"

    Component {
        id: componentMyObject

        MyObject {

        }
    }

    Component {
        id: componentSpy

        SignalSpy {

        }
    }

    function test_increment() {
        const myObject = createTemporaryObject(componentMyObject, null, {});
        compare(myObject.incrementNumber(1), 2);
    }

    function test_increment_self() {
        const myObject = createTemporaryObject(componentMyObject, null, {
            number: 1,
        });
        myObject.incrementNumberSelf();
        compare(myObject.number, 2);
    }

    function test_number() {
        const myObject = createTemporaryObject(componentMyObject, null, {
            number: 1,
        });
        const spy = createTemporaryObject(componentSpy, null, {
            signalName: "numberChanged",
            target: myObject,
        });
        compare(myObject.number, 1);
        compare(spy.count, 0);
        // Wait for init value to be set
        tryCompare(spy, "count", 1);

        myObject.number = 2;

        compare(myObject.number, 2);
        tryCompare(spy, "count", 2);
    }

    function test_string() {
        const myObject = createTemporaryObject(componentMyObject, null, {
            string: "hello",
        });
        const spy = createTemporaryObject(componentSpy, null, {
            signalName: "stringChanged",
            target: myObject,
        });
        compare(myObject.string, "hello");
        compare(spy.count, 0);
        // Wait for init value to be set
        tryCompare(spy, "count", 1);

        myObject.string = "world";

        compare(myObject.string, "world");
        tryCompare(spy, "count", 2);
    }
}
