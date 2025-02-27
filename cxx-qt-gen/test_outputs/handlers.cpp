#include "cxx-qt-gen/include/my_object.cxxqt.h"

namespace cxx_qt::my_object {

MyObject::MyObject(QObject* parent)
  : QObject(parent)
  , m_rustObj(cxx_qt::my_object::cxx_qt_my_object::createRs())
{
  cxx_qt::my_object::cxx_qt_my_object::initialiseCpp(*this);
  m_initialised = true;
}

MyObject::~MyObject() = default;

const MyObjectRust&
MyObject::unsafeRust() const
{
  return *m_rustObj;
}

MyObjectRust&
MyObject::unsafeRustMut()
{
  return *m_rustObj;
}

qint32
MyObject::getNumber() const
{
  return m_number;
}

void
MyObject::setNumber(qint32 value)
{
  if (!m_initialised) {
    m_number = value;
    return;
  }

  if (value != m_number) {
    m_number = value;

    const auto signalSuccess =
      QMetaObject::invokeMethod(this, "numberChanged", Qt::QueuedConnection);
    Q_ASSERT(signalSuccess);
  }
}

const QString&
MyObject::getString() const
{
  return m_string;
}

void
MyObject::setString(const QString& value)
{
  if (!m_initialised) {
    m_string = value;
    return;
  }

  if (value != m_string) {
    m_string = value;

    const auto signalSuccess =
      QMetaObject::invokeMethod(this, "stringChanged", Qt::QueuedConnection);
    Q_ASSERT(signalSuccess);
  }
}

std::unique_ptr<rust::cxxqtlib1::UpdateRequester>
MyObject::updateRequester()
{
  return std::make_unique<rust::cxxqtlib1::UpdateRequester>(this,
                                                            "updateState");
}

void
MyObject::updateState()
{
  const std::lock_guard<std::mutex> guard(m_rustObjMutex);
  m_rustObj->handleUpdateRequest(*this);
}

} // namespace cxx_qt::my_object

namespace cxx_qt::my_object::cxx_qt_my_object {
std::unique_ptr<MyObject>
newCppObject()
{
  return std::make_unique<MyObject>();
}
} // namespace cxx_qt::my_object::cxx_qt_my_object
