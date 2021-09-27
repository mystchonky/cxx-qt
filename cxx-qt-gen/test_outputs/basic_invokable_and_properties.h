#pragma once

#include "rust/cxx_qt.h"

namespace cxx_qt::my_object {

class RustObj;

class MyObject : public CxxQObject
{
  Q_OBJECT
  Q_PROPERTY(qint32 number READ getNumber WRITE setNumber NOTIFY numberChanged)
  Q_PROPERTY(QString string READ getString WRITE setString NOTIFY stringChanged)

public:
  explicit MyObject(QObject* parent = nullptr);
  ~MyObject();

  qint32 getNumber() const;
  const QString& getString() const;

  Q_INVOKABLE void sayHi(const QString& string, qint32 number);
  Q_INVOKABLE void sayBye();

public Q_SLOTS:
  void setNumber(qint32 value);
  void setString(const QString& value);

Q_SIGNALS:
  void numberChanged();
  void stringChanged();

private:
  rust::Box<RustObj> m_rustObj;
  bool m_initialised = false;

  qint32 m_number;
  QString m_string;
};

std::unique_ptr<MyObject>
newCppObject();

} // namespace cxx_qt::my_object
