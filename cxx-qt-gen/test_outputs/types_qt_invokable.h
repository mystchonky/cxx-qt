#pragma once

#include "rust/cxx_qt.h"

#include <QtCore/QPointF>
#include <QtCore/QVariant>

namespace cxx_qt::my_object {

class RustObj;

class MyObject : public CxxQObject
{
  Q_OBJECT

public:
  explicit MyObject(QObject* parent = nullptr);
  ~MyObject();

  Q_INVOKABLE QPointF testPointf(const QPointF& pointf);
  Q_INVOKABLE QString testString(const QString& string);
  Q_INVOKABLE QVariant testVariant(const QVariant& variant);

private:
  rust::Box<RustObj> m_rustObj;
  bool m_initialised = false;
};

std::unique_ptr<MyObject>
newCppObject();

} // namespace cxx_qt::my_object
