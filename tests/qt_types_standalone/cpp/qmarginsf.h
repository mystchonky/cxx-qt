// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QMarginsF>
#include <QtTest/QTest>

#include "cxx-qt-gen/qmarginsf_cxx.cxx.h"

class QMarginsFTest : public QObject
{
  Q_OBJECT

private Q_SLOTS:
  void construct()
  {
    const auto m = construct_qmarginsf();
    QCOMPARE(m.left(), 1.0);
    QCOMPARE(m.top(), 2.0);
    QCOMPARE(m.right(), 3.0);
    QCOMPARE(m.bottom(), 4.0);
  }

  void read()
  {
    const auto m = QMarginsF(1.0, 2.0, 3.0, 4.0);
    QVERIFY(read_qmarginsf(m));
  }

  void clone()
  {
    const auto m = QMargins(1.0, 2.0, 3.0, 4.0);
    const auto c = clone_qmarginsf(m);
    QCOMPARE(c.left(), 1.0);
    QCOMPARE(c.top(), 2.0);
    QCOMPARE(c.right(), 3.0);
    QCOMPARE(c.bottom(), 4.0);
  }
};
