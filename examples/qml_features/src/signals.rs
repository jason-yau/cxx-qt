// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: book_macro_code
#[cxx_qt::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/include/qt_types.h");
        type QPoint = cxx_qt_lib::QPoint;
        type QVariant = cxx_qt_lib::QVariant;
    }

    // ANCHOR: book_signals_enum
    #[cxx_qt::signals(Signals)]
    pub enum Signal {
        Ready,
        RustDataChanged { data: i32 },
        TrivialDataChanged { trivial: QPoint },
        OpaqueDataChanged { opaque: UniquePtr<QVariant> },
    }
    // ANCHOR_END: book_signals_enum

    pub struct Data {
        data: i32,
        trivial: QPoint,
        opaque: UniquePtr<QVariant>,
    }

    impl Default for Data {
        fn default() -> Self {
            Self {
                data: 0,
                trivial: QPoint::default(),
                opaque: QVariant::null(),
            }
        }
    }

    #[cxx_qt::qobject]
    #[derive(Default)]
    pub struct Signals;

    // ANCHOR: book_rust_obj_impl
    impl cxx_qt::QObject<Signals> {
        #[qinvokable]
        pub fn invokable(&self, cpp: &mut CppObj) {
            unsafe {
                cpp.emit_immediate(Signal::Ready);
            }

            cpp.emit_queued(Signal::RustDataChanged { data: cpp.data() });
            cpp.emit_queued(Signal::TrivialDataChanged {
                trivial: *cpp.trivial(),
            });
            cpp.emit_queued(Signal::OpaqueDataChanged {
                opaque: QVariant::from_ref(cpp.opaque()),
            });
        }
    }
    // ANCHOR_END: book_rust_obj_impl
}
// ANCHOR_END: book_macro_code
