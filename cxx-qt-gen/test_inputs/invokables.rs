#[cxx_qt::bridge(namespace = "cxx_qt::my_object")]
mod ffi {
    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/include/qt_types.h");
        type QColor = cxx_qt_lib::QColor;
        type QPoint = cxx_qt_lib::QPoint;
        type QString = cxx_qt_lib::QString;
    }

    #[cxx_qt::qobject]
    #[derive(Default)]
    pub struct MyObject;

    impl cxx_qt::QObject<MyObject> {
        #[qinvokable]
        pub fn invokable(&self) {
            println!("invokable");
        }

        #[qinvokable]
        pub fn invokable_cpp_obj(&self, cpp: &mut CppObj) {
            println!("cppobj");
        }

        #[qinvokable]
        pub fn invokable_mutable(&mut self) {
            println!("This method is mutable!");
        }

        #[qinvokable]
        pub fn invokable_mutable_cpp_obj(&mut self, cpp: &mut CppObj) {
            println!("This method is mutable!");
        }

        #[qinvokable]
        pub fn invokable_parameters(&self, opaque: &QColor, trivial: &QPoint, primitive: i32) {
            println!(
                "Red: {}, Point X: {}, Number: {}",
                opaque.red(),
                trivial.x(),
                primitive,
            );
        }

        #[qinvokable]
        pub fn invokable_parameters_cpp_obj(&self, primitive: i32, cpp: &mut CppObj) {
            println!("{}", primitive);
        }

        #[qinvokable]
        pub fn invokable_return_opaque(&mut self) -> UniquePtr<QColor> {
            cxx_qt_lib::QColor::from_rgba(255, 0, 0, 0)
        }

        #[qinvokable]
        pub fn invokable_return_primitive(&mut self) -> i32 {
            2
        }

        #[qinvokable]
        pub fn invokable_return_static(&mut self) -> UniquePtr<QString> {
            QString::from_str("static")
        }

        pub fn cpp_context_method(&self) {
            println!("C++ context method");
        }

        pub fn cpp_context_method_mutable(&mut self) {
            println!("mutable method");
        }

        pub fn cpp_context_method_cpp_obj(&mut self, cpp: &mut CppObj) {
            println!("cppobj");
        }

        pub fn cpp_context_method_return_opaque(&self) -> UniquePtr<QColor> {
            cxx_qt_lib::QColor::from_rgba(255, 0, 0, 0)
        }
    }

    impl MyObject {
        pub fn rust_only_method(&self) {
            println!("QML or C++ can't call this :)");
        }
    }
}
