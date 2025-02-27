// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::generator::cpp::{fragment::CppFragmentPair, GeneratedCppBlocks};
use indoc::formatdoc;

/// Extract the source from a given CppFragmentPair
fn pair_as_source(pair: &CppFragmentPair) -> String {
    pair.source.clone()
}

/// For a given GeneratedCppBlocks write this into a C++ source
pub fn write_cpp_source(generated: &GeneratedCppBlocks) -> String {
    formatdoc! {r#"
        #include "cxx-qt-gen/include/{cxx_stem}.cxxqt.h"

        {namespace_start}

        {ident}::{ident}(QObject* parent)
          : QObject(parent)
          , m_rustObj({namespace_internals}::createRs())
        {{
          {namespace_internals}::initialiseCpp(*this);
          m_initialised = true;
        }}

        {ident}::~{ident}() = default;

        const {rust_ident}&
        {ident}::unsafeRust() const
        {{
          return *m_rustObj;
        }}

        {rust_ident}&
        {ident}::unsafeRustMut()
        {{
          return *m_rustObj;
        }}

        {methods}
        {slots}
        {namespace_end}

        namespace {namespace_internals} {{
        std::unique_ptr<{ident}>
        newCppObject()
        {{
          return std::make_unique<{ident}>();
        }}
        }} // namespace {namespace_internals}
    "#,
    cxx_stem = generated.cxx_stem,
    ident = generated.ident,
    namespace_start = if generated.namespace.is_empty() {
      "".to_owned()
    } else {
      format!("namespace {namespace} {{", namespace = generated.namespace)
    },
    namespace_end = if generated.namespace.is_empty() {
      "".to_owned()
    } else {
      format!("}} // namespace {namespace}", namespace = generated.namespace)
    },
    namespace_internals = generated.namespace_internals,
    rust_ident = generated.rust_ident,
    methods = generated.methods.iter().map(pair_as_source).collect::<Vec<String>>().join("\n"),
    slots = generated.slots.iter().map(pair_as_source).collect::<Vec<String>>().join("\n"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::writer::cpp::tests::{
        create_generated_cpp, create_generated_cpp_no_namespace, expected_source,
        expected_source_no_namespace,
    };
    use pretty_assertions::assert_str_eq;

    #[test]
    fn test_write_cpp_source() {
        let generated = create_generated_cpp();
        let output = write_cpp_source(&generated);
        assert_str_eq!(output, expected_source());
    }

    #[test]
    fn test_write_cpp_source_no_namespace() {
        let generated = create_generated_cpp_no_namespace();
        let output = write_cpp_source(&generated);
        assert_str_eq!(output, expected_source_no_namespace());
    }
}
